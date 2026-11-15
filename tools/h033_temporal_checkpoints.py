#!/usr/bin/env python3
"""H033: immutable temporal checkpoints with late / out-of-order witnesses.

Synthetic evidence study only; source-root independence flags are assertions in
fixtures, NOT external authentication. Append-only hash chain detects tampering
with retained records, but does not make a malicious publisher immutable.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path

BASE = 120
TOTAL = BASE**3
WHITE = (0, 31, 62, 93, 124)
GENESIS = "0"*64


def digest(value):
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"),
                         ensure_ascii=True).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


class Journal:
    def __init__(self):
        self.events = []
        self.frozen = {}

    def append(self, kind, payload):
        if kind not in ("WITNESS_RECEIVED", "CHECKPOINT_FROZEN",
                        "FREEZE_WITHHELD", "LATE_CONFIRMATION",
                        "REVISION_PROPOSED"):
            raise ValueError("unknown event kind")
        entry = {"seq": len(self.events), "kind": kind,
                 "previous": self.events[-1]["hash"] if self.events else GENESIS,
                 "payload": copy.deepcopy(payload)}
        entry["hash"] = digest(entry)
        self.events.append(entry)
        return copy.deepcopy(entry)

    def receive(self, observation):
        """Received order is log seq. observed_tick is event time, not arrival time."""
        o = copy.deepcopy(observation)
        validate(o)
        receipt = self.append("WITNESS_RECEIVED", o)
        tick = o["observed_tick"]
        if tick in self.frozen:
            frozen = self.frozen[tick]
            if o["rgb"] == frozen["rgb"]:
                self.append("LATE_CONFIRMATION", {
                    "observed_tick": tick, "receipt_seq": receipt["seq"],
                    "frozen_hash": frozen["frozen_hash"],
                    "root_ancestry": o["root_ancestry"],
                    "note": "confirmation recorded without mutating frozen state"})
            else:
                self.append("REVISION_PROPOSED", {
                    "observed_tick": tick, "receipt_seq": receipt["seq"],
                    "frozen_hash": frozen["frozen_hash"],
                    "candidate_rgb": o["rgb"],
                    "original_rgb": frozen["rgb"],
                    "status": "DISPUTED_NO_AUTO_REPAIR"})
        return receipt

    def freeze(self, tick):
        if type(tick) is not int or tick < 0 or tick >= TOTAL:
            raise ValueError("invalid tick")
        if tick in self.frozen:
            raise ValueError("checkpoint already frozen; no overwrite")
        candidates = [e["payload"] for e in self.events
                      if e["kind"] == "WITNESS_RECEIVED"
                      and e["payload"]["observed_tick"] == tick]
        # Group by actual asserted ancestry (not by copy filename or mirror URL).
        ancestry = {}
        for item in candidates:
            ancestry.setdefault(item["root_ancestry"], []).append(item)
        conflicting = any(
            len({tuple(x["rgb"]) for x in group}) != 1
            or len({x["claimed_witness"] for x in group}) != 1
            for group in ancestry.values())
        states = {tuple(x["rgb"]) for x in candidates}
        all_verified = all(x["independence_attested"] is True
                           for x in candidates)
        eligible = (len(ancestry) >= 3 and not conflicting
                    and len(states) == 1 and all_verified)
        if not eligible:
            status = ("CONTRADICTION" if conflicting or len(states) > 1
                      else "INSUFFICIENT_ROOTS" if len(ancestry) < 3
                      else "UNVERIFIED_ANCESTRY")
            return self.append("FREEZE_WITHHELD", {
                "observed_tick": tick, "status": status,
                "source_ancestries": sorted(ancestry)})
        state = next(iter(states))
        event = self.append("CHECKPOINT_FROZEN", {
            "observed_tick": tick, "rgb": list(state),
            "source_ancestries": sorted(ancestry),
            "witness_count": len(candidates),
            "independent_ancestries_claimed": len(ancestry),
            "independence_verified_externally": False,
            "frozen_at_sequence": len(self.events),
            "status": "FROZEN_SYNTHETIC_PRECONDITIONS"})
        self.frozen[tick] = {"rgb": list(state), "frozen_hash": event["hash"],
                             "seq": event["seq"]}
        return event

    def verify(self):
        previous = GENESIS
        reconstructed = {}
        for i, e in enumerate(self.events):
            if e["seq"] != i or e["previous"] != previous:
                return False
            if e["hash"] != digest({k:v for k,v in e.items() if k != "hash"}):
                return False
            if e["kind"] == "CHECKPOINT_FROZEN":
                t = e["payload"]["observed_tick"]
                if t in reconstructed:
                    return False
                reconstructed[t] = {"rgb": e["payload"]["rgb"],
                                    "frozen_hash": e["hash"], "seq": i}
            previous = e["hash"]
        return reconstructed == self.frozen


def validate(o):
    if type(o.get("observed_tick")) is not int or not 0 <= o["observed_tick"] < TOTAL:
        raise ValueError("invalid event tick")
    rgb = o.get("rgb")
    if not isinstance(rgb, (list,tuple)) or len(rgb) != 3 or any(
            type(x) is not int or x < 0 or x >= BASE for x in rgb):
        raise ValueError("invalid RGB state")
    if not isinstance(o.get("root_ancestry"), str) or not o["root_ancestry"]:
        raise ValueError("source ancestry required")
    if not isinstance(o.get("claimed_witness"), str) or not o["claimed_witness"]:
        raise ValueError("claimed witness required")
    if type(o.get("independence_attested")) is not bool:
        raise ValueError("explicit provenance assertion required")


def obs(tick, rgb, root, attested=True, name=None):
    return {"observed_tick": tick, "rgb": list(rgb),
            "root_ancestry": root, "claimed_witness": name or root,
            "independence_attested": attested}


def run_scenario(scenario):
    j = Journal()
    tick = 14399
    state = [119,119,0]
    def three(t, s):
        for root in ("A","B","C"):
            j.receive(obs(t,s,root))
    if scenario == "late_agreement":
        three(tick,state)
        frozen = j.freeze(tick)
        original = copy.deepcopy(j.frozen[tick])
        j.receive(obs(tick,state,"D"))
        assert j.frozen[tick] == original
    elif scenario == "late_contradiction":
        three(tick,state)
        frozen = j.freeze(tick)
        original = copy.deepcopy(j.frozen[tick])
        j.receive(obs(tick,[0,119,0],"D"))
        assert j.frozen[tick] == original
    elif scenario == "arrival_out_of_order":
        # Later measured tick arrives first; two different immutable checkpoints.
        three(tick+1,[0,0,1])
        frozen = j.freeze(tick+1)
        three(tick,state)
        j.freeze(tick)
    elif scenario == "shared_ancestry":
        for label in ("GitHub","0root.ai","replica"):
            j.receive(obs(tick,state,"owned-corpus",name="same-source"))
        frozen = j.freeze(tick)
    elif scenario == "pre_freeze_contradiction":
        j.receive(obs(tick,state,"A"))
        j.receive(obs(tick,state,"B"))
        j.receive(obs(tick,[0,119,0],"C"))
        frozen = j.freeze(tick)
    elif scenario == "unverified":
        j.receive(obs(tick,state,"A"))
        j.receive(obs(tick,state,"B"))
        j.receive(obs(tick,state,"C",False))
        frozen = j.freeze(tick)
    else:
        raise ValueError("invalid scenario")
    assert j.verify()
    statuses=[e["kind"] for e in j.events]
    summary={"scenario":scenario,
             "event_count":len(j.events),"event_types":statuses,
             "frozen_ticks":sorted(j.frozen),
             "frozen":{str(k):v for k,v in sorted(j.frozen.items())},
             "append_only_verified":True,
             "last_event_hash":j.events[-1]["hash"],
             "white_axis_reserved":list(WHITE),
             "late_revision_count":statuses.count("REVISION_PROPOSED"),
             "late_confirmation_count":statuses.count("LATE_CONFIRMATION"),
             "freeze_withheld_count":statuses.count("FREEZE_WITHHELD")}
    return summary, j


def run():
    scenarios=("late_agreement","late_contradiction","arrival_out_of_order",
               "shared_ancestry","pre_freeze_contradiction","unverified")
    results=[run_scenario(s)[0] for s in scenarios]
    assert [len(s["frozen_ticks"]) for s in results] == [1,1,2,0,0,0]
    assert results[1]["late_revision_count"] == 1
    assert results[0]["late_confirmation_count"] == 1
    return {"schema":"cern.h033.temporal-checkpoint.v1",
            "status":"synthetic evidence study; source independence not authenticated",
            "scenarios":results,
            "immutability_rule":"Late evidence appends; it cannot overwrite a frozen state or its digest",
            "observed_time_rule":"observed_tick is event time; seq is receipt order",
            "scientific_boundary":"Not measurements of CERN particles, Higgs or magnets."}


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--output",default="models/h033-temporal-results.json")
    args=parser.parse_args()
    result=run()
    path=Path(args.output)
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8")
    print([(s["scenario"],s["frozen_ticks"],s["late_revision_count"])
           for s in result["scenarios"]])


if __name__ == "__main__":
    main()
