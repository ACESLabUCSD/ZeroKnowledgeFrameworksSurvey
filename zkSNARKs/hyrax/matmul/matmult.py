#!/usr/bin/python
#
# (C) 2017 Riad S. Wahby <rsw@cs.nyu.edu>
#
# generate a matrix multiply PWS

# Modified from https://github.com/hyraxZK/pws/blob/master/matmult.py for Python3

import sys

def print_dp(r, c, matdim, voffset, fh):
    # dot product in row-major format
    dpvals = []
    matsize = matdim * matdim
    fh.write("// row %d, col %d\n" % (r, c))
    for dpval in range(0, matdim):
        rowidx = r * matdim + dpval
        colidx = dpval * matdim + c + matsize
        fh.write("P V%d = V%d * V%d E\n" % (voffset, rowidx, colidx))
        dpvals.append(voffset)
        voffset += 1

    # add tree
    while len(dpvals) > 1:
        outs = []
        for idx in range(0, len(dpvals)//2):
            fh.write("P V%d = V%d + V%d E\n" % (voffset, dpvals[2*idx], dpvals[2*idx+1]))
            outs.append(voffset)
            voffset += 1

        if len(dpvals) % 2 != 0:
            outs.append(dpvals[-1])

        dpvals = outs

    return (dpvals[0], voffset)

def print_pws(matdim, fh):
    # inputs
    voffset = 2 * matdim * matdim
    for inidx in range(0, voffset):
        fh.write("P V%d = I%d E\n" % (inidx, inidx))

    outs = []
    for r in range(0, matdim):
        for c in range(0, matdim):
            (out, voffset) = print_dp(r, c, matdim, voffset, fh)
            outs.append(out)

    # write output
    for outidx in outs:
        fh.write("P O%d = V%d E\n" % (voffset, outidx))
        voffset += 1

def print_rdl(matdim, fh):
    # RDL routes inputs (in row-major format)

    # inputs
    voffset = 2 * matdim * matdim
    for inidx in range(0, voffset):
        fh.write("P V%d = I%d E\n" % (inidx, inidx))

    # wire inputs to subckts
    for nrow in range(0, matdim):
        row_offset = nrow * matdim
        for ncol in range(0, matdim):
            # write the row inputs
            for ridx in range(0, matdim):
                row_ent = row_offset + ridx
                fh.write("P V%d = V%d PASS V%d E\n" % (voffset, row_ent, row_ent))
                voffset += 1
            # write the column inputs
            for cidx in range(0, matdim):
                col_ent = ncol + cidx * matdim
                fh.write("P V%d = V%d PASS V%d E\n" % (voffset, col_ent, col_ent))
                voffset += 1

    # outputs
    assert voffset == 2 * matdim * matdim + 2 * matdim * matdim * matdim
    for oval in range(0, 2 * matdim * matdim * matdim):
        fh.write("P O%d = V%d E\n" % (voffset, oval + 2 * matdim * matdim))
        voffset += 1

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: %s <n>" % sys.argv[0])
        sys.exit(-1)

    md = int(sys.argv[1])
    with open("matmult_%d.pws" % md, 'w') as f:
        print_pws(md, f)