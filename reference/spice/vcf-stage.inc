* Reconciled VCF: Q11 replaced by imposed measured ladder current.
* External signal sources do not change the ladder/output/feedback topology.
.subckt VCF input out tap tailcontrol params: RES=0.5
R95 rail vr 100
V12 rail 0 12
Vref ref 0 5.333
C28 vr 0 47u
R115 vr b22 10k
C30 b22 0 10u
Q22l vr b22 l1 N303
Q22r vr b22 r1 N303
Q16 l1 l1 l2 N303
Q23 r1 r1 r2 N303
Q14 l2 l2 l3 N303
Q17 r2 r2 r3 N303
Q13 l3 l3 l4 N303
Q15 r3 r3 r4 N303
C26 l1 r1 33n
C24 l2 r2 33n
C19 l3 r3 33n
C18 l4 r4 18n
Q12l l4 bl emit N303
Q12r r4 br emit N303
R69 emit sink 2.2k
Btail sink 0 I=V(tailcontrol)
R62 input c17 220k
C17 c17 bl 1u
R70 bl bi 2.2k
R68 ref bi 2.2k
C16 bi 0 10u
R98 bi br 2.2k
R97 br c23 10k
C23 c23 fbe 1u
C25 l1 b21r .1u
C27 r1 b21l .1u
R113 b21l bd 100k
R114 b21r bd 100k
R116 ref bd 10k
C29 bd 0 1u
R112 bd b20 10k
R109 b20 0 10k
Q20 tail b20 e20 N303
R108 e20 0 2.2k
Q21l cl b21l tail N303
Q21r cr b21r tail N303
R111 vr cl 2.2k
R110 vr cr 2.2k
Q19 vr cr e19 N303
R96 e19 0 10k
C14 e19 out 1u
Rvr4a_hi out tap {max(1,50000*(1-RES))}
Rvr4a_lo tap 0 {max(1,50000*RES)}
C15 tap c15 1u
R67 c15 b18 2.2k
R99 b18 bf 100k
R66 ref bf 100k
C22_vcf bf 0 1u
Q18 rail b18 fbe N303
R94 fbe 0 10k
* VCA input loading, AC bias approximately stiff for isolated VCF measurements.
C21_vca out n121 10n
R121 n121 vca_in 220k
C22_vca tap n122 10n
R122 n122 vca_in 100k
R124 vca_in 0 2.2k
.ends
