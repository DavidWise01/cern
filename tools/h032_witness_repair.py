#!/usr/bin/env python3
"""H032: three independent witness roots, contradiction gate, append-only repair.

This is a synthetic test harness, NOT a CERN instrument interface or proof
that arbitrary real-world source roots are independent.
"""
import argparse
import hashlib
import json
from pathlib import Path

BASE=120
MOD=BASE**3
WHITE=(0,31,62,93,124)
GENESIS="0"*64
OBS_FIELDS=("tick","R","B","G")


def encode(rgb):
    if len(rgb)!=3 or any(type(x)!=int or x<0 or x>=BASE for x in rgb):
        raise ValueError("invalid RGB state")
    return rgb[0]+BASE*rgb[1]+BASE*BASE*rgb[2]


def decode(value):
    if type(value)!=int or value<0 or value>=MOD:
        raise ValueError("invalid address")
    return [value%BASE,value//BASE%BASE,value//(BASE*BASE)]


def canonical(value):
    return json.dumps(value,sort_keys=True,separators=(",",":"),ensure_ascii=True)


def hash_event(prior,kind,payload):
    return hashlib.sha256(canonical({
        "prior":prior,"kind":kind,"payload":payload
    }).encode("utf-8")).hexdigest()


def append(ledger,kind,payload):
    prior=ledger[-1]["digest"] if ledger else GENESIS
    event={"sequence":len(ledger),"prior_digest":prior,"kind":kind,
           "payload":payload,"digest":hash_event(prior,kind,payload)}
    ledger.append(event)
    return event


def verify_chain(ledger):
    prev=GENESIS
    for i,event in enumerate(ledger):
        if event["sequence"]!=i or event["prior_digest"]!=prev:
            return False
        if event["digest"]!=hash_event(prev,event["kind"],event["payload"]):
            return False
        prev=event["digest"]
    return True


def observation(tick,rgb,root,source_id,ancestry,verified=False):
    if not isinstance(root,str) or not root: raise ValueError("root required")
    if not isinstance(ancestry,str) or not ancestry: raise ValueError("ancestry required")
    return {"tick":tick,"R":rgb[0],"B":rgb[1],"G":rgb[2],
            "claimed_root":root,"source_id":source_id,
            "ancestry_id":ancestry,"independence_verified":verified}


def evaluate(observations):
    """One ancestry = at most one root, regardless of copies.

    Distinct verified ancestry assertions are a simulator precondition only.
    Exact tick and all three coordinates must agree. Contradictions veto.
    """
    if not observations:
        return {"decision":"INSUFFICIENT","independent_roots":0,
                "candidate":None,"independence_limit":"no observations"}
    for o in observations:
        if not isinstance(o.get("tick"),int):
            return {"decision":"INVALID","independent_roots":0,"candidate":None}
        encode([o["R"],o["B"],o["G"]])
        if not o.get("ancestry_id") or not o.get("claimed_root"):
            return {"decision":"INVALID","independent_roots":0,"candidate":None}
    grouped={}
    for o in observations:
        key=o["ancestry_id"]
        if key not in grouped:grouped[key]=[]
        grouped[key].append(o)
    roots=[]
    for ancestry,copies in grouped.items():
        payloads={tuple(c[field] for field in OBS_FIELDS) for c in copies}
        # Even conflicting reports from one claimed ancestry fail closed.
        if len(payloads)>1:
            return {"decision":"CONTRADICTION","independent_roots":len(grouped),
                    "candidate":None,"reason":"single ancestry supplies conflicting observations"}
        if len({c["claimed_root"] for c in copies})>1:
            return {"decision":"PROVENANCE_CONFLICT","independent_roots":len(grouped),
                    "candidate":None}
        roots.append({"ancestry":ancestry,"payload":next(iter(payloads)),
                      "verified":all(c.get("independence_verified") is True for c in copies)})
    values={root["payload"] for root in roots}
    if len(values)>1:
        decision="CONTRADICTION"
    elif len(roots)<3:
        decision="INSUFFICIENT"
    elif not all(r["verified"] for r in roots):
        decision="UNVERIFIED_ROOTS"
    else:
        decision="CONSENSUS_IN_SIMULATOR"
    return {"decision":decision,"independent_roots":len(roots),
            "candidate":dict(zip(OBS_FIELDS,next(iter(values)))) if len(values)==1 else None,
            "independence_limit":"verified flag is provided by test fixture; no real source identity is cryptographically authenticated"}


def trial(scenario):
    tick=14399
    reference=decode(tick)
    faulty=[0,119,0] # Red digit 119 advanced without correct carry
    assert reference==[119,119,0]
    obs=[
        observation(tick,reference,"witness-A","original-A","root-A",True),
        observation(tick,reference,"witness-B","original-B","root-B",True),
        observation(tick,reference,"witness-C","original-C","root-C",True)
    ]
    if scenario=="mirrors":
        obs[1]=observation(tick,reference,"witness-A","github-copy","root-A",True)
        obs[2]=observation(tick,reference,"witness-A","website-copy","root-A",True)
    if scenario=="contradiction":
        obs[2]=observation(tick,faulty,"witness-C","original-C","root-C",True)
    if scenario=="unverified":
        obs[2]["independence_verified"]=False
    ledger=[]
    append(ledger,"FAULT",{"tick":tick,"observed_rgb":faulty,"fault_band":"R"})
    for x in obs:append(ledger,"WITNESS",x)
    decision=evaluate(obs)
    append(ledger,"ASSESSMENT",decision)
    did_repair=decision["decision"]=="CONSENSUS_IN_SIMULATOR"
    if did_repair:
        candidate=decision["candidate"]
        if candidate["tick"]!=tick:raise AssertionError("time mismatch")
        correction=(encode([candidate["R"],candidate["B"],candidate["G"]])-encode(faulty))%MOD
        corrected=decode((encode(faulty)+correction)%MOD)
        assert corrected==reference
        append(ledger,"CORRECTION",{"tick":tick,"before":faulty,"after":corrected,
                                   "delta_modulo":correction,
                                   "reference_assessment_digest":ledger[-1]["digest"]})
    else:
        corrected=faulty
        append(ledger,"REPAIR_WITHHELD",{"reason":decision["decision"],"state":corrected})
    assert verify_chain(ledger)
    return {"scenario":scenario,"decision":decision["decision"],
            "independent_roots":decision["independent_roots"],
            "reference":reference,"fault":faulty,"final_state":corrected,
            "repaired":did_repair,"white_reserved":list(WHITE),
            "event_count":len(ledger),"chain_valid":True,
            "first_event_digest":ledger[0]["digest"],
            "last_event_digest":ledger[-1]["digest"],"events":ledger}


def run():
    cases=[trial(x) for x in ("agreement","mirrors","contradiction","unverified")]
    assert [x["repaired"] for x in cases]==[True,False,False,False]
    assert all(x["chain_valid"] and x["white_reserved"]==list(WHITE) for x in cases)
    return {"schema":"cern.h032.witness-gated-repair.v1",
            "status":"synthetic deterministic local simulation, no authenticated external witnesses",
            "rules":["Three separately rooted and externally verified observations required",
                     "All reports must match tick, R, B, G exactly",
                     "Copies sharing ancestry are not independent",
                     "One dissenting verified source blocks repair",
                     "Original fault and all witness evidence are retained in a hash-linked append-only ledger",
                     "Repair requires a reference and is appended, not retroactively written"],
            "cases":cases,"note":"Test fixture's independence_verified=true is a simulated premise; not empirical or cryptographic independent corroboration."}


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--output",default="models/h032-witness-gated-results.json")
    args=parser.parse_args()
    output=Path(args.output)
    output.parent.mkdir(parents=True,exist_ok=True)
    doc=run()
    output.write_text(json.dumps(doc,indent=2)+"\n",encoding="utf-8")
    print("H032",[(c["scenario"],c["decision"],c["repaired"]) for c in doc["cases"]])


if __name__=="__main__":
    main()
