from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json

N=30; EPS=F(1,64); A=F(3,64); COST=A*A
DELAYS=(0,1,2)
CHANGES=((9,19),(7,16,24),(12,))
NOISES=((F(0),),(-EPS,F(0),EPS),(EPS,-EPS))
SIGNALS=('pulses','ramps','flat')
POLICIES=('never','periodic','always','residual')
Q=F(1,128)
LATENCY=2

def trajectory(signal):
    x=[F(0)]
    for t in range(N):
        if signal=='pulses': u=A if t in (3,8,14,20,26) else F(0)
        elif signal=='ramps': u=F((t//5)%3-1,32)
        else: u=F(0)
        x.append(x[-1]+F(1,128)-u)
    return x

def delay_schedule(d0, changes):
    values=(d0,(d0+1)%3,(d0+2)%3,d0)
    return [values[sum(t>=c for c in changes)] for t in range(N)]

def run(d0,change,noise,signal,policy):
    x=trajectory(signal);actual=delay_schedule(d0,change)
    y=[x[max(0,t-actual[t])]+noise[t%len(noise)] for t in range(N)]
    stored=d0;pending=None;probes=alarms=false_alarms=misses=0;stale=0;wrong_results=0
    first_detect={c:None for c in change}
    for t in range(N):
        if pending is not None and pending[0]==t:
            _,sample_time,sample_delay=pending
            stored=sample_delay;pending=None
            if sample_delay!=actual[t]:wrong_results+=1
        discrepancy=abs(y[t]-x[max(0,t-stored)])
        alarm=discrepancy>EPS
        if alarm:
            alarms+=1
            if stored==actual[t]:false_alarms+=1
            for c in change:
                if c<=t and first_detect[c] is None and stored!=actual[t]: first_detect[c]=t
        elif stored!=actual[t]:misses+=1
        should=(policy=='always' and t%2==0 or policy=='periodic' and t%6==0 or policy=='residual' and alarm)
        if should and pending is None and t+LATENCY < N:
            pending=(t+LATENCY,t,actual[t]);probes+=1
        if stored!=actual[t]:stale+=1
    observed=[t-c for c,t in first_detect.items() if t is not None]
    return dict(probes=probes,alarms=alarms,false_alarms=false_alarms,
                missed_stale_readings=misses,stale_steps=stale,
                detected_changes=len(observed),total_changes=len(change),
                mean_detection_latency=F(sum(observed),len(observed)) if observed else None,
                stale_probe_replies=wrong_results,
                total_cost=probes*COST+stale*Q)

def main():
    cases={};totals={p:{'cases':0,'probes':0,'false_alarms':0,'stale_steps':0,'detected_changes':0,'total_changes':0,'missed_stale_readings':0,'stale_probe_replies':0,'total_cost':F(0)} for p in POLICIES}
    for d0,changes,noise,signal,policy in product(DELAYS,CHANGES,NOISES,SIGNALS,POLICIES):
        v=run(d0,changes,noise,signal,policy)
        cases[(d0,changes,noise,signal,policy)]=v
        agg=totals[policy];agg['cases']+=1
        for k in ('probes','false_alarms','stale_steps','detected_changes','total_changes','missed_stale_readings','stale_probe_replies','total_cost'):agg[k]+=v[k]
        assert v['false_alarms']==0
        assert v['probes']*COST+v['stale_steps']*Q==v['total_cost']
    comparisons={}
    for other in POLICIES[:-1]:
        w=t=l=0
        for d0,changes,noise,signal in product(DELAYS,CHANGES,NOISES,SIGNALS):
            a=cases[(d0,changes,noise,signal,'residual')]['total_cost']
            b=cases[(d0,changes,noise,signal,other)]['total_cost']
            w+=a<b;t+=a==b;l+=a>b
        comparisons[other]={'wins':w,'ties':t,'losses':l}
    result={'schema':'cern.h188.unannounced-switch.v1','cases':len(cases),'updates':len(cases)*N,'noise_bound':str(EPS),'diagnostic_pulse_cost':str(COST),'stale_cost_per_step':str(Q),
        'policies':{p:{k:str(x) if isinstance(x,F) else x for k,x in ag.items()} for p,ag in totals.items()},'matched_comparisons':comparisons,
        'limitations':['Requires known accurate excitation and phase trajectory for residual checks','Noise bound must hold; false alarm 0 is conditional, not universal','Probe response delayed 2 steps, may already be stale on arrival','Signal flat does not guarantee delay observability','No externally announced changes or oracle-triggered probing','Synthetic timing-cost model; not physical Rupert/CMB power']}
    Path(__file__).with_name('results.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))
if __name__=='__main__':main()
