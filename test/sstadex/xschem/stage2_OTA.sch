v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 70 -120 70 -100 {
lab=#net1}
N 70 -40 70 -20 {
lab=vout}
N 200 -120 200 -110 {
lab=#net1}
N 70 -120 200 -120 {
lab=#net1}
N 200 -110 200 -100 {
lab=#net1}
N 200 -40 200 -20 {
lab=vout}
N 70 -20 200 -20 {
lab=vout}
N 40 -120 70 -120 {
lab=#net1}
N 30 -120 40 -120 {
lab=#net1}
N -50 -120 -50 -90 {
lab=Vaout}
N 70 -20 70 30 {
lab=vout}
N 70 110 70 130 {
lab=vss}
N 70 -250 70 -120 {
lab=#net1}
N -170 -120 -70 -120 {
lab=Vaout}
N -330 -120 -280 -120 {
lab=Vaout}
N -280 -120 -250 -120 {
lab=Vaout}
N -250 -120 -220 -120 {
lab=Vaout}
N -330 -50 -330 -40 {
lab=vss}
N -330 -40 -330 -20 {
lab=vss}
N -330 -40 -210 -40 {
lab=vss}
N -210 -50 -210 -40 {
lab=vss}
N -210 -120 -210 -110 {
lab=Vaout}
N 70 10 240 10 {
lab=vout}
N -330 -120 -330 -110 {
lab=Vaout}
N -220 -120 -170 -120 {
lab=Vaout}
N 240 10 250 10 {
lab=vout}
N -50 -50 30 -50 {
lab=Vaout}
N -50 -90 -50 -50 {
lab=Vaout}
N 30 -120 30 -90 {
lab=#net1}
N -70 -120 -50 -120 {
lab=Vaout}
N 170 -280 170 -250 {
lab=#net1}
N 70 -280 170 -280 {
lab=#net1}
N 70 -280 70 -260 {
lab=#net1}
N 70 -260 70 -250 {
lab=#net1}
N 170 -240 170 -230 {
lab=#net1}
N 170 -250 170 -240 {
lab=#net1}
N -250 -100 -250 -60 {
lab=V_n}
N -290 -100 -250 -100 {
lab=V_n}
N -250 -60 -250 -10 {
lab=V_n}
N 70 30 70 40 {
lab=vout}
N 70 100 70 110 {
lab=vss}
N -290 40 -290 90 {
lab=V_p}
N -290 -60 -290 40 {
lab=V_p}
N -20 10 70 10 {
lab=vout}
N 160 10 160 70 {
lab=vout}
N 160 140 160 160 {
lab=vss}
N 160 130 160 140 {
lab=vss}
N -120 -120 -120 -110 {
lab=Vaout}
N -120 -50 -120 -40 {
lab=vss}
N -210 -40 -120 -40 {
lab=vss}
C {devices/res.sym} 200 -70 0 0 {name=Ro_stage2
value=473596.7
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 70 -70 2 1 {name=Gm_stage2 value=2.7200000000000004e-05}
C {devices/vccs.sym} -330 -80 2 0 {name=Gma_1stage value=10}
C {devices/res.sym} -210 -80 2 0 {name=Ra_1stage
value=50
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -290 -60 2 0 {name=p4 sig_type=std_logic lab=V_p}
C {devices/lab_pin.sym} -330 -20 3 0 {name=p8 sig_type=std_logic lab=vss
value=vref}
C {devices/vsource.sym} 170 -200 0 0 {name=V1 value=3 savecurrent=false}
C {devices/lab_pin.sym} 170 -170 3 0 {name=p1 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -250 20 0 0 {name=V3 value=3 savecurrent=false}
C {devices/lab_pin.sym} -250 50 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 70 130 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 250 10 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/isource.sym} 70 70 0 0 {name=I2 value=40e-6}
C {devices/vsource.sym} -290 120 0 0 {name=V2 value=3 savecurrent=false}
C {devices/lab_pin.sym} -290 150 3 0 {name=p5 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -250 -20 2 0 {name=p7 sig_type=std_logic lab=V_n}
C {capa.sym} 160 100 0 0 {name=CL
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -20 -20 0 0 {name=C2
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/lab_pin.sym} 160 160 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -50 -120 0 1 {name=p10 sig_type=std_logic lab=Vaout}
C {capa.sym} -120 -80 0 0 {name=Ca_1stage
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
