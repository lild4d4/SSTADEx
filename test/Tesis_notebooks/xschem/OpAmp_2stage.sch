v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -770 -140 -650 -140 {
lab=#net1}
N -770 -70 -770 -60 {
lab=vss}
N -650 -70 -650 -60 {
lab=vss}
N -650 -140 -650 -130 {
lab=#net1}
N -770 -140 -770 -130 {
lab=#net1}
N -730 -120 -690 -120 {
lab=vss}
N -690 -60 -650 -60 {
lab=vss}
N -690 -120 -690 -60 {
lab=vss}
N -770 -60 -690 -60 {
lab=vss}
N -730 -80 -730 -30 {
lab=vpos}
N -400 -140 -280 -140 {
lab=vout}
N -400 -70 -400 -60 {
lab=vss}
N -280 -70 -280 -60 {
lab=vss}
N -280 -140 -280 -130 {
lab=vout}
N -400 -140 -400 -130 {
lab=vout}
N -400 -60 -280 -60 {
lab=vss}
N -650 -140 -510 -140 {
lab=#net1}
N -510 -140 -510 -130 {
lab=#net1}
N -510 -70 -510 -60 {
lab=vss}
N -440 -60 -400 -60 {
lab=vss}
N -510 -140 -440 -140 {
lab=#net1}
N -440 -140 -440 -120 {
lab=#net1}
N -440 -80 -440 -60 {
lab=vss}
N -510 -60 -440 -60 {
lab=vss}
N -280 -140 -160 -140 {
lab=vout}
N -160 -140 -160 -130 {
lab=vout}
N -280 -60 -160 -60 {
lab=vss}
N -160 -70 -160 -60 {
lab=vss}
N -650 -60 -510 -60 {
lab=vss}
N -400 -250 -400 -140 {
lab=vout}
N -420 -250 -400 -250 {
lab=vout}
N -510 -250 -480 -250 {
lab=#net2}
N -510 -250 -510 -230 {
lab=#net2}
N -510 -170 -510 -140 {
lab=#net1}
C {devices/vccs.sym} -770 -100 0 1 {name=Gma_1stage value=1}
C {devices/res.sym} -650 -100 2 0 {name=Ra_1stage
value=100
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -650 -60 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -160 -140 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/vsource.sym} -730 0 0 0 {name=Vpos value=1 savecurrent=false}
C {devices/lab_pin.sym} -730 30 3 0 {name=p8 sig_type=std_logic lab=vss
value=1}
C {devices/lab_pin.sym} -730 -50 0 0 {name=p5 sig_type=std_logic lab=vpos}
C {capa.sym} -160 -100 0 0 {name=Cin_pt
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -510 -100 0 0 {name=Cin_2stage
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/vccs.sym} -400 -100 0 0 {name=Gma_2stage value=1}
C {devices/res.sym} -280 -100 2 0 {name=Ra_2stage
value=100
footprint=1206
device=resistor
m=1}
C {capa.sym} -450 -250 3 0 {name=Cgd_2stage
m=1
value=5e-14
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} -510 -200 2 0 {name=Rc
value=100
footprint=1206
device=resistor
m=1}
