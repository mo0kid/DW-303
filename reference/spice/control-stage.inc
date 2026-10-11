* Actual Q9/Q10/Q11 converter and loaded panel tracks. Fractions are electrical.
* VERIFY: A taper mapping rotation -> fraction = (100^rotation-1)/99.
* VERIFY: gangs A/B move toward terminal 3 together as assumed by transcription.
.subckt CONTROL env accent current sweep c13 params: CUT=.5 MOD=0 RES=.5 TM3=235k
V12 rail 0 12
Vref ref 0 5.333
R65 ref b9 10k
R64 b9 0 10k
Q9 0 b9 vq9 P303
Q10 vq9 eb ee P303
Q11 collector ee vq9 N303
* VERIFY: stiff 12V Q11 collector ignores actual R69/Q12 compliance;
* converter currents are available demand, not guaranteed delivered current.
Vsense rail collector 0
Bcurrent current 0 V=I(Vsense)
R71 eb vq9 2.2k
R73 ee tm 100k
Rtm3 tm cw {max(1,TM3)}
Rcut_hi rail cw {max(1,50000*(1-(pow(100,CUT)-1)/99))}
Rcut_lo cw cutlo {max(1,50000*(pow(100,CUT)-1)/99)}
R47 cutlo vq9 10k
Rmod_hi env mw {max(1,50000*(1-(pow(100,MOD)-1)/99))}
Rmod_lo mw modlo {max(1,50000*(pow(100,MOD)-1)/99)}
R61 modlo vq9 10k
R63 mw eb 220k
D24 accent d24 D303
R46 d24 reslo 47k
Rres_lo reslo sweep {max(1,50000*RES)}
Rres_hi sweep c13 {max(1,50000*(1-RES))}
C13 c13 0 1u
R72 sweep eb 100k
.ends
