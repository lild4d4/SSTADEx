v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 70 -130 70 -110 {
lab=#net1}
N 70 -50 70 -30 {
lab=vout}
N 200 -130 200 -120 {
lab=#net1}
N 70 -130 200 -130 {
lab=#net1}
N 200 -120 200 -110 {
lab=#net1}
N 200 -50 200 -30 {
lab=vout}
N 70 -30 200 -30 {
lab=vout}
N 40 -130 70 -130 {
lab=#net1}
N 30 -130 40 -130 {
lab=#net1}
N -50 -130 -50 -100 {
lab=Vaout}
N 70 -30 70 20 {
lab=vout}
N 70 100 70 120 {
lab=vss}
N 70 -260 70 -130 {
lab=#net1}
N -170 -130 -70 -130 {
lab=Vaout}
N -330 -130 -280 -130 {
lab=Vaout}
N -280 -130 -250 -130 {
lab=Vaout}
N -250 -130 -220 -130 {
lab=Vaout}
N -330 -60 -330 -50 {
lab=vss}
N -330 -50 -330 -30 {
lab=vss}
N -330 -50 -210 -50 {
lab=vss}
N -210 -60 -210 -50 {
lab=vss}
N -210 -130 -210 -120 {
lab=Vaout}
N 70 0 240 0 {
lab=vout}
N -330 -130 -330 -120 {
lab=Vaout}
N -220 -130 -170 -130 {
lab=Vaout}
N 240 0 250 0 {
lab=vout}
N -50 -60 30 -60 {
lab=Vaout}
N -50 -100 -50 -60 {
lab=Vaout}
N 30 -130 30 -100 {
lab=#net1}
N -70 -130 -50 -130 {
lab=Vaout}
N 170 -290 170 -260 {
lab=#net1}
N 70 -290 170 -290 {
lab=#net1}
N 70 -290 70 -270 {
lab=#net1}
N 70 -270 70 -260 {
lab=#net1}
N 170 -250 170 -240 {
lab=#net1}
N 170 -260 170 -250 {
lab=#net1}
N -250 -110 -250 -70 {
lab=V_n}
N -290 -110 -250 -110 {
lab=V_n}
N -250 -70 -250 -20 {
lab=V_n}
N 70 20 70 30 {
lab=vout}
N 70 90 70 100 {
lab=vss}
N -290 30 -290 80 {
lab=V_p}
N -290 -70 -290 30 {
lab=V_p}
N -20 0 70 0 {
lab=vout}
N 160 0 160 60 {
lab=vout}
N 160 130 160 150 {
lab=vss}
N 160 120 160 130 {
lab=vss}
N -120 -130 -120 -120 {
lab=Vaout}
N -120 -60 -120 -50 {
lab=vss}
N -210 -50 -120 -50 {
lab=vss}
N 160 150 250 150 {
lab=vss}
N 250 -0 250 10 {
lab=vout}
N 250 70 250 90 {
lab=vout}
C {devices/res.sym} 200 -80 0 0 {name=Ro_stage2
value=473596.7
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 70 -80 2 1 {name=Gm_stage2 value=2.7200000000000004e-05}
C {devices/vccs.sym} -330 -90 2 0 {name=Gma_1stage value=10}
C {devices/res.sym} -210 -90 2 0 {name=Ra_1stage
value=50
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -290 -70 2 0 {name=p4 sig_type=std_logic lab=V_p}
C {devices/lab_pin.sym} -330 -30 3 0 {name=p8 sig_type=std_logic lab=vss
value=vref}
C {devices/vsource.sym} 170 -210 0 0 {name=V1 value=3 savecurrent=false}
C {devices/lab_pin.sym} 170 -180 3 0 {name=p1 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -250 10 0 0 {name=V3 value=3 savecurrent=false}
C {devices/lab_pin.sym} -250 40 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 70 120 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 250 0 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/isource.sym} 70 60 0 0 {name=I2 value=40e-6}
C {devices/vsource.sym} -290 110 0 0 {name=V2 value=3 savecurrent=false}
C {devices/lab_pin.sym} -290 140 3 0 {name=p5 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -250 -30 2 0 {name=p7 sig_type=std_logic lab=V_n}
C {capa.sym} 160 90 0 0 {name=CL
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -20 -30 0 0 {name=C2
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 160 150 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -50 -130 0 1 {name=p10 sig_type=std_logic lab=Vaout}
C {capa.sym} -120 -90 0 0 {name=Ca_1stage
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/vsource.sym} 250 120 0 0 {name=V4 value=3 savecurrent=false}
C {devices/res.sym} 250 40 2 0 {name=Ra_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 250 80 2 0 {name=p11 sig_type=std_logic lab=vr}
