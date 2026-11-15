#!/usr/bin/env python3
"""H031 — isolated single-ring fault versus reference clock, and controlled repair.

The H030 update is a translation on Z/(120^3). An injected phase error remains
constant under identical subsequent updates; a nominal orbit does NOT self-heal.
The repair routine requires a verified reference checkpoint (outside this toy
simulator). Synthetic witness labels do not independently authenticate anything.
"""
import argparse
import json
from pathlib import Path

BASE=120
TOTAL=BASE**3
WHITE=(0,31,62,93,124)
WEIGHTS={"R":1,"B":BASE,"G":BASE**2}
DEFAULT_TICK=14399


def encode(state):
    if not isinstance(state, (tuple,list)) or len(state)!=3 or any(type(n)!=int or not 0<=n<BASE for n in state):
        raise ValueError("state must be three integers in [0,119]")
    return state[0]+BASE*state[1]+BASE**2*state[2]


def decode(address):
    if type(address)!=int or not 0<=address<TOTAL:
        raise ValueError("address out of range")
    return (address%BASE,(address//BASE)%BASE,address//(BASE**2))


def advance(state,ticks=1):
    if type(ticks)!=int or ticks<0: raise ValueError("ticks must be nonnegative integer")
    return decode((encode(state)+ticks)%TOTAL)


def inject(state,band,delta=1):
    if band not in WEIGHTS: raise ValueError("band must be R, B or G")
    if type(delta)!=int: raise ValueError("delta must be integer")
    vals=list(state)
    idx="RBG".index(band)
    vals[idx]=(vals[idx]+delta)%BASE
    return tuple(vals)


def signed_minimal_offset(fault,reference):
    n=(encode(fault)-encode(reference))%TOTAL
    return n if n<=TOTAL//2 else n-TOTAL


def repair(fault,reference,verified=False):
    """Only use a checkpoint that was independently verified outside this model."""
    if verified is not True: raise PermissionError("Unverified reference checkpoint")
    correction=(-signed_minimal_offset(fault,reference))%TOTAL
    corrected=decode((encode(fault)+correction)%TOTAL)
    assert corrected==tuple(reference)
    return corrected,correction


def experiment(tick=DEFAULT_TICK,horizon=240):
    if type(tick)!=int or not 0<=tick<TOTAL or type(horizon)!=int or horizon<0:
        raise ValueError("invalid time/horizon")
    original=decode(tick)
    results=[]
    for band in ("R","B","G"):
        fault=inject(original,band)
        delta=(encode(fault)-encode(original))%TOTAL
        samples=[]
        for elapsed in sorted(set([0,1,2,119,120,121,239,240,horizon])):
            ref=advance(original,elapsed)
            disturbed=advance(fault,elapsed)
            residual=(encode(disturbed)-encode(ref))%TOTAL
            assert residual==delta
            assert disturbed!=ref
            samples.append({"elapsed":elapsed,"reference":list(ref),"disturbed":list(disturbed),"residual_address_modulo":residual})
        denied=False
        try: repair(fault,original)
        except PermissionError: denied=True
        assert denied
        corrected,adjust=repair(fault,original,verified=True)
        assert corrected==original
        assert all(advance(corrected,n)==advance(original,n) for n in (0,1,120,360,horizon))
        results.append({
            "band":band,"injected_at_tick":tick,"original":list(original),
            "disturbed_at_injection":list(fault),"injected_delta":1,
            "integer_address_offset_modulo":delta,
            "integer_address_offset_signed":signed_minimal_offset(fault,original),
            "spontaneous_recovery":False,"phase_divergence_persists_forever_without_feedback":True,
            "unverified_repair_blocked":denied,
            "verified_reference_repair_possible":True,
            "repair_address_shift_modulo":adjust,
            "checkpoint_independence":"not demonstrated; verified=True is an explicit authorization supplied by caller",
            "observations":samples
        })
    return {"schema":"cern.h031.single-ring-disturbance.v1",
            "model":"translation-invariant base-120 carry counter",
            "fault_tick":tick,"reference_state":list(original),"horizon":horizon,
            "perturbation":"add one to one ring's digit without propagating a carry (single-event register fault)",
            "white_reservations_fixed":list(WHITE),
            "cases":results,"theorem":"F(x)=x+1 mod 120^3 => F^t(x+δ)-F^t(x)=δ mod 120^3",
            "interpretation":"No intrinsic self-correction. Explicit reference-based repair needed; repair is not evidence of independent witnesses.",
            "synthetic":True,"cern_measurement":False}


def main():
    p=argparse.ArgumentParser()
    p.add_argument("--tick",type=int,default=DEFAULT_TICK)
    p.add_argument("--horizon",type=int,default=240)
    p.add_argument("--output",default="models/h031-disturbance-results.json")
    args=p.parse_args()
    results=experiment(args.tick,args.horizon)
    output=Path(args.output);output.parent.mkdir(parents=True,exist_ok=True)
    output.write_text(json.dumps(results,indent=2)+"\n",encoding="utf-8")
    print("H031: all three single-register faults persisted; verified-reference repair restored them.")


if __name__=="__main__":
    main()
