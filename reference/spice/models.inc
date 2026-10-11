* Generic device substitutes at 27 C. Not manufacturer models.
* VERIFY: Is/beta/Early voltage/junction capacitances/transit times for every BJT.
* N303 represents 2SC1583/2SC2291 matched pairs and unspecified small NPNs.
.model N303 NPN(Is=1e-14 Bf=200 Br=4 Vaf=100 Ikf=.1 Cje=8p Cjc=4p Tf=.4n Tr=20n)
* P303 represents 2SA733 and unspecified small PNPs.
.model P303 PNP(Is=1e-14 Bf=150 Br=4 Vaf=80 Ikf=.05 Cje=12p Cjc=6p Tf=.6n Tr=30n)
* VERIFY: generic Si diode; per-designator part assignment unknown.
.model D303 D(Is=2n N=1.7 Rs=1 Cjo=2p Tt=4n)
* VERIFY: 2SK30 Y/O substitutes: Idss=2mA/1mA, Vto=-1.5V; rank curves unknown.
.model JY NJF(Beta=.000888889 Vto=-1.5 Lambda=.01)
.model JO NJF(Beta=.000444444 Vto=-1.5 Lambda=.01)
* VERIFY: idealized 12V 4066, Ron=200 ohm, no charge injection.
.model SW303 SW(Ron=200 Roff=1e12 Vt=6 Vh=.1)
* VERIFY: AN6562 idealized high-gain servo with rail clipping, no offset/GBW.
.subckt OP303 plus minus out
Bop out 0 V=max(0,min(12,6+1e6*V(plus,minus)))
.ends
* VERIFY: BA662A LM13700-style differential pair law. Pin1=1.2V assumed;
* gm=Iabc/(2VT), current limiting=tanh; ideal unity output buffer.
* This is NOT a vendor BA662A model; ignores bleed, offset and buffer loading.
.subckt OTA303 plus minus abc current_out buffer_out
Vabc abc 0 1.2
Bota 0 current_out I=max(0,I(Vabc))*tanh(V(plus,minus)/.05173)
Bbuf buffer_out 0 V=max(.2,min(11.8,V(current_out)))
.ends
