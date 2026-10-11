* Actual one-shot / MEG / Darlington / accent switches.
* Gate inverter falling edge provided externally; CPU latch not emulated.
.subckt ENV ngate act pulse meg env acc params: DECAY=.5 ACCENT=1
V12 rail 0 12
Vref ref 0 5.333
C54 ngate n54 47n
R148 n54 b38 10k
R149 rail b38 10k
R150 rail e38 22
C55 e38 0 47u
Q38 pulse b38 e38 P303
R151 pulse 0 22k
D37 pulse d37 D303
R152 d37 meg 100
C62 meg 0 1u
R138 meg decay 68k
Rvr6 decay declo {max(1,1e6*(pow(100,DECAY)-1)/99)}
Sdecay decay declo act 0 SW303
D28 declo d28 D303
D27_env d28 dref D303
D26 dref 0 D303
R117 ref dref 22k
Q41 rail meg b40 N303
Q40 rail b40 env N303
J39 env 0 0 JY
Sacc env acctop act 0 SW303
Rvr7hi acctop acc {max(1,50000*(1-ACCENT))}
Rvr7lo acc 0 {max(1,50000*ACCENT)}
.ends
