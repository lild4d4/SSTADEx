v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -450 -100 -330 -100 {
lab=#net1}
N -450 -30 -450 -20 {
lab=vss}
N -330 -30 -330 -20 {
lab=vss}
N -330 -100 -330 -90 {
lab=#net1}
N -450 -100 -450 -90 {
lab=#net1}
N -410 -80 -370 -80 {
lab=vss}
N -370 -20 -330 -20 {
lab=vss}
N -370 -80 -370 -20 {
lab=vss}
N -450 -20 -370 -20 {
lab=vss}
N -410 -40 -410 10 {
lab=vpos}
N -80 -100 40 -100 {
lab=vout}
N -80 -30 -80 -20 {
lab=vss}
N 40 -30 40 -20 {
lab=vss}
N 40 -100 40 -90 {
lab=vout}
N -80 -100 -80 -90 {
lab=vout}
N -80 -20 40 -20 {
lab=vss}
N -190 -100 -190 -90 {
lab=#net1}
N -190 -30 -190 -20 {
lab=vss}
N -120 -20 -80 -20 {
lab=vss}
N -190 -100 -120 -100 {
lab=#net1}
N -120 -100 -120 -80 {
lab=#net1}
N -120 -40 -120 -20 {
lab=vss}
N -190 -20 -120 -20 {
lab=vss}
N 40 -100 160 -100 {
lab=vout}
N 160 -100 160 -90 {
lab=vout}
N 40 -20 160 -20 {
lab=vss}
N 160 -30 160 -20 {
lab=vss}
N -330 -100 -190 -100 {
lab=#net1}
N -330 -20 -190 -20 {
lab=vss}
N -190 -210 -170 -210 {
lab=#net2}
N -110 -210 -80 -210 {
lab=vout}
N -80 -210 -80 -100 {
lab=vout}
N -190 -210 -190 -190 {
lab=#net2}
N -190 -130 -190 -100 {
lab=#net1}
C {devices/vccs.sym} -450 -60 0 1 {name=Gma_1stage value=0.0003277}
C {devices/res.sym} -330 -60 2 0 {name=Ra_1stage
value=108549
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -330 -20 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 160 -100 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -410 10 0 0 {name=p5 sig_type=std_logic lab=vpos}
C {capa.sym} 160 -60 0 0 {name=Cin_pt
m=1
value=2.5e-11
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -190 -60 0 0 {name=Cin_2stage
m=1
value=1.678e-14
footprint=1206
device="ceramic capacitor"}
C {devices/vccs.sym} -80 -60 0 0 {name=Gma_2stage value=0.00013814}
C {devices/res.sym} 40 -60 2 0 {name=Ra_2stage
value=993887
footprint=1206
device=resistor
m=1}
C {devices/code_shown.sym} -190 180 0 0 {name=s1 only_toplevel=false value="

Vpos vpos 0 0 ac 1
Vss vss 0 0

.control

ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI


.endc

"}
C {capa.sym} -140 -210 3 0 {name=Cin_1
m=1
value=3e-10
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} -190 -160 2 0 {name=Rc
value=7000
footprint=1206
device=resistor
m=1}
