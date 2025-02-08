#!/usr/bin/python
#
# (C) 2017 Riad S. Wahby <rsw@cs.nyu.edu>
#
# run Fennel given a PWS file

import bz2
import getopt
import resource
import sys
import time
import traceback

try:
    import pypws
except ImportError:
    print("ERROR: could not import pypws; you should run `make`. Giving up now.")
    sys.exit(1)

import libfennel.circuitni
import libfennel.circuitnizk
import libfennel.circuitnizkvecwit
import libfennel.commit
from libfennel.defs import Defs
from libfennel.fiatshamir import FiatShamir
import libfennel.parse_pws
import libfennel.randutil as randutil
import libfennel.util as util

class VerifierInfo(object):
    nCopyBits = 1
    nCopies = 2

    curve = 'm191'

    ndBits = None
    ndGen = None

    rvStart = None
    rvEnd = None

    pwsFile = None
    rdlFile = None

    inputFile = None

    pProofFile = None
    vProofFile = None

    proofType = libfennel.circuitnizkvecwit
    witnessDiv = 2

    logFile = None

    showPerf = False
    pStartTime = 0
    pEndTime = 0
    vStartTime = 0
    vEndTime = 0

    class Log(object):
        logLines = []

        @classmethod
        def get_lines(cls):
            for line in cls.logLines:
                yield line
                yield '\n'

        @classmethod
        def log(cls, lStr, do_print):
            cls.logLines.append(lStr)
            if do_print:
                print(lStr)

def get_usage():
    uStr =  "Usage: %s -p <pwsFile> [options]\n\n" % sys.argv[0]
    uStr += " option        description                                 default\n"
    uStr += " --            --                                          --\n"

    uStr += " -p pwsFile    PWS describing the computation to run.      (None)\n\n"

    uStr += " -r rdlFile    use RDL from file                           (None)\n\n"

    uStr += " -c nCopyBits  log2(#copies) to verify in parallel         (%d)\n" % VerifierInfo.nCopyBits
    uStr += "    NOTE: \"-c =<nCopies>\" treats arg as #copies instead of log2\n\n"

    uStr += " -i inputsFile file containing inputs for each copy        (None)\n"
    uStr += "               (otherwise, inputs are generated at random\n\n"

    uStr += " -n b          Set nondet input range per sub-AC.          (None)\n"
    uStr += "               The final 2^(-b) fraction of input indices\n"
    uStr += "               are part of the witness (e.g., x=1 means the\n"
    uStr += "               last 1/2 of input is the witness, x=2 means 1/4, etc.\n\n"

    uStr += " -R x,y        (inclusive) rvalue range per sub-AC         (None,None)\n\n"

    uStr += " -g genscript  set filename of nondet generator script, a  (None)\n"
    uStr += "               .py file that defines a function like so:\n"
    uStr += "  def nondet_gen(inputs, muxsels):\n"
    uStr += "      new_inputs = []\n"
    uStr += "      for (copy_num, copy_ins) in enumerate(inputs):\n"
    uStr += "          new_copy_ins = []\n"
    uStr += "          # do something with this copy's inputs\n"
    uStr += "          new_inputs.append(new_copy_ins)\n"
    uStr += "      return new_inputs\n\n"

    uStr += " -o prooffile  write proof to prooffile                    (None)\n"
    uStr += "               (cannot be used at the same time as -v)\n\n"

    uStr += " -v prooffile  verify proof from prooffile                 (None)\n"
    uStr += "               (cannot be used at the same time as -o)\n\n"

    uStr += " NOTE: If neither -o nor -v is supplied, generates and verifies proof.\n\n"

    uStr += " -z <0|1|2|3>  0: noninteractive                           (3)\n"
    uStr += "               1: noninteractive zk, elementwise commits\n"
    uStr += "               2: REMOVED\n"
    uStr += "               3: noninteractive zk, vecs for sumcheck and witness\n\n"

    uStr += " -w n          In nizkvecwit mode, have P send |w|^(1/n)   (2)\n"
    uStr += "               commits in phase 1. Default is n=2.\n"
    uStr += "               If n=1, does not use log commit in 2nd phase.\n\n"

    uStr += " -C <curve>    use curve <curve> (m159, m191, m221, m255)  (%s)\n\n" % VerifierInfo.curve
    uStr += " -P            use prime field                             (%s)\n\n" % str(VerifierInfo.curve is None)
    uStr += " -T            show performance info when done             (False)\n\n"

    uStr += " -L <logfile>  log perf info to logfile                    (None)\n"

    return uStr

def get_inputs(verifier_info, input_layer):
    nCopies = verifier_info.nCopies if verifier_info.rdlFile is None else 1

    if verifier_info.inputFile is None:
        inputs = randutil.rand_inputs(0, nCopies, input_layer)
    else:
        inputs = util.get_inputs(verifier_info.inputFile, input_layer, nCopies)

    if len(inputs) != nCopies:
        print("ERROR: input file has too few lines (got %d, expected %d)" % (len(inputs), nCopies))
        sys.exit(1)

    return inputs

def run_fennel(verifier_info):
    # set curve and prime
    Defs.curve = verifier_info.curve
    util.set_prime(libfennel.commit.MiraclEC.get_order(Defs.curve))

    # pylint doesn't seed to understand how classmethods are inherited from metclasses
    p_from_pws = verifier_info.proofType.ProverClass.from_pws # pylint: disable=no-member
    v_from_pws = verifier_info.proofType.VerifierClass.from_pws # pylint: disable=no-member
    pFile = pypws.parse_pws(verifier_info.pwsFile, str(Defs.prime))

    # handle RDL
    if verifier_info.rdlFile is not None:
        rFile = pypws.parse_pws_unopt(verifier_info.rdlFile, str(Defs.prime))
        (r_input_layer, rdl_map) = libfennel.parse_pws.parse_rdl(rFile, verifier_info.nCopies, pFile[0])

    # either generate or read in proof
    if verifier_info.vProofFile is None:
        (input_layer, prv) = p_from_pws(pFile, verifier_info.nCopies)
        prv.build_prover()

        # set up RDL
        if verifier_info.rdlFile is not None:
            inputs = get_inputs(verifier_info, r_input_layer)
            prv.set_rdl(rdl_map, len(r_input_layer))
        else:
            inputs = get_inputs(verifier_info, input_layer)

        # handle nondeterminism options
        if verifier_info.ndBits is not None:
            prv.set_nondet_range(verifier_info.ndBits)
        if verifier_info.ndGen is not None:
            prv.set_nondet_gen(verifier_info.ndGen)
        if verifier_info.rvStart is not None and verifier_info.rvEnd is not None:
            prv.set_rval_range(verifier_info.rvStart, verifier_info.rvEnd)
        if verifier_info.witnessDiv is not None:
            prv.set_wdiv(verifier_info.witnessDiv)

        verifier_info.pStartTime = time.time()
        proof = prv.run(inputs)
        verifier_info.pEndTime = time.time()
    else:
        with open(verifier_info.vProofFile, 'r') as fh:
            proof = bz2.decompress(fh.read())

    verifier_info.Log.log("Proof size: %d elems, %d bytes" % FiatShamir.proof_size(proof), True)

    # either verify or write out proof
    if verifier_info.pProofFile is None:
        (_, ver) = v_from_pws(pFile, verifier_info.nCopies)

        # set up RDL
        if verifier_info.rdlFile is not None:
            ver.set_rdl(rdl_map, len(r_input_layer))

        verifier_info.vStartTime = time.time()
        try:
            ver.run(proof)
        except Exception as e: # pylint: disable=broad-except
            verifier_info.Log.log("Verification failed: %s" % e, True)
            verifier_info.Log.log(traceback.format_exc(), True)
        else:
            verifier_info.Log.log("Verification succeeded.", True)
        verifier_info.vEndTime = time.time()
    else:
        with open(verifier_info.pProofFile, 'w') as fh:
            fh.write(bz2.compress(proof))

    nInBits = util.clog2(len(input_layer))
    if verifier_info.rdlFile is not None:
        nInBits = util.clog2(len(r_input_layer))
    nCopies = verifier_info.nCopies
    nLayers = len(ver.in0vv) + 1 if verifier_info.rdlFile is not None else 0
    verifier_info.Log.log("nInBits: %d, nCopies: %d, nLayers: %d" % (nInBits, nCopies, nLayers), verifier_info.showPerf)
    if Defs.track_fArith:
        verifier_info.Log.log(str(Defs.fArith()), verifier_info.showPerf)

def main():
    uStr = get_usage()
    oStr = "c:i:p:n:g:o:v:C:Pz:Tr:R:L:w:"

    try:
        (opts, args) = getopt.getopt(sys.argv[1:], oStr)
    except getopt.GetoptError as err:
        print(uStr)
        print(str(err))
        sys.exit(1)

    if args:
        print(uStr)
        print("ERROR: extraneous arguments.")
        sys.exit(1)

    for (opt, arg) in opts:
        if opt == "-c":
            if arg[0] == "=":
                nC = int(arg[1:])
                nCB = util.clog2(nC)
            else:
                nCB = int(arg)
                nC = 1 << nCB
            VerifierInfo.nCopyBits = nCB
            VerifierInfo.nCopies = nC
        elif opt == "-i":
            VerifierInfo.inputFile = arg
        elif opt == "-p":
            VerifierInfo.pwsFile = arg
        elif opt == "-r":
            VerifierInfo.rdlFile = arg
        elif opt == "-n":
            VerifierInfo.ndBits = int(arg)
        elif opt == "-R":
            (VerifierInfo.rvStart, VerifierInfo.rvEnd) = [ int(x) for x in arg.split(',') ]
        elif opt == "-g":
            arg_ns = {}
            try:
                execfile(arg, arg_ns)
                VerifierInfo.ndGen = staticmethod(arg_ns['nondet_gen'])
            except IOError:
                print(uStr)
                print("ERROR: Could not open file %s" % arg)
                sys.exit(1)
            except NameError:
                print(uStr)
                print("ERROR: Could not find nondet_gen function in file %s" % arg)
                sys.exit(1)
        elif opt == "-v":
            VerifierInfo.vProofFile = arg
        elif opt == "-o":
            VerifierInfo.pProofFile = arg
        elif opt == "-C":
            VerifierInfo.curve = arg
        elif opt == "-P":
            VerifierInfo.curve = None
        elif opt == "-z":
            arg = int(arg)
            if arg == 0:
                VerifierInfo.proofType = libfennel.circuitni
            elif arg == 1:
                VerifierInfo.proofType = libfennel.circuitnizk
            elif arg == 2:
                raise ValueError("circuitnizkvec no longer exists")
            elif arg == 3:
                VerifierInfo.proofType = libfennel.circuitnizkvecwit
            else:
                assert False, "got '-z %s', expected 0, 1, or 2" % arg
            if arg != 3:
                VerifierInfo.witnessDiv = None
        elif opt == "-T":
            VerifierInfo.showPerf = True
        elif opt == "-L":
            VerifierInfo.logFile = arg
        elif opt == "-w":
            if arg == '1':
                VerifierInfo.witnessDiv = None
            else:
                VerifierInfo.witnessDiv = float(arg)
        else:
            assert False, "logic error: got unexpected option %s from getopt" % opt

    if VerifierInfo.pwsFile is None:
        print(uStr)
        print("ERROR: missing required argument, -p <pwsFile>.")
        sys.exit(1)

    if VerifierInfo.nCopyBits < 1:
        print(uStr)
        print("ERROR: nCopyBits must be at least 1.")
        sys.exit(1)

    if VerifierInfo.vProofFile is not None and VerifierInfo.pProofFile is not None:
        print(uStr)
        print("ERROR: only one of -o and -v can be supplied.")
        sys.exit(1)

    if VerifierInfo.witnessDiv is not None:
        if VerifierInfo.proofType is not libfennel.circuitnizkvecwit:
            print("WARNING: -w can only be specified for -z 3. Ignoring.")
            VerifierInfo.witnessDiv = None

    run_fennel(VerifierInfo)

    VerifierInfo.Log.log("Prover runtime: %f" % (VerifierInfo.pEndTime - VerifierInfo.pStartTime), VerifierInfo.showPerf)
    VerifierInfo.Log.log("Verifier runtime: %f" % (VerifierInfo.vEndTime - VerifierInfo.vStartTime), VerifierInfo.showPerf)
    VerifierInfo.Log.log("Max memory usage: %d kB" % resource.getrusage(resource.RUSAGE_SELF).ru_maxrss, VerifierInfo.showPerf)

    if VerifierInfo.logFile is not None:
        with open(VerifierInfo.logFile, 'w') as fh:
            fh.writelines(VerifierInfo.Log.get_lines())

if __name__ == "__main__":
    main()