v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 70 -130 70 -110 {
lab=vdd}
N 70 -50 70 -30 {
lab=vout}
N 200 -130 200 -110 {
lab=vdd}
N 70 -130 200 -130 {
lab=vdd}
N 200 -50 200 -30 {
lab=vout}
N 70 -30 200 -30 {
lab=vout}
N 30 -130 70 -130 {
lab=vdd}
N -50 -130 -50 -60 {
lab=#net1}
N 70 0 70 20 {
lab=vout}
N 70 90 70 110 {
lab=vfb}
N 70 180 70 190 {
lab=vss}
N -420 -130 -300 -130 {
lab=#net1}
N -420 -60 -420 -50 {
lab=vss}
N -300 -60 -300 -50 {
lab=vss}
N -300 -130 -300 -120 {
lab=#net1}
N 30 90 70 90 {
lab=vfb}
N -420 -130 -420 -120 {
lab=#net1}
N 30 -130 30 -100 {
lab=vdd}
N 170 -290 170 -240 {
lab=vdd}
N 70 -290 170 -290 {
lab=vdd}
N -380 -110 -340 -110 {
lab=#net2}
N -50 -60 30 -60 {
lab=#net1}
N 290 0 330 0 {
lab=vout}
N 290 0 290 70 {
lab=vout}
N 190 180 290 180 {
lab=vss}
N 290 130 290 180 {
lab=vss}
N 70 80 70 90 {
lab=vfb}
N 70 -30 70 0 {
lab=vout}
N 190 -0 290 0 {
lab=vout}
N 70 170 70 180 {
lab=vss}
N 70 -290 70 -130 {
lab=vdd}
N -420 -50 -300 -50 {
lab=vss}
N 20 -130 30 -130 {
lab=vdd}
N -50 -130 -40 -130 {
lab=#net1}
N 20 -30 70 -30 {
lab=vout}
N -50 -30 -40 -30 {
lab=#net1}
N -50 -60 -50 -30 {
lab=#net1}
N -230 -130 -230 -120 {
lab=#net1}
N -230 -130 -50 -130 {
lab=#net1}
N -300 -130 -230 -130 {
lab=#net1}
N -300 -50 -230 -50 {
lab=vss}
N -230 -60 -230 -50 {
lab=vss}
N 190 -0 190 70 {
lab=vout}
N 70 0 190 -0 {
lab=vout}
N 190 130 190 180 {
lab=vss}
N 70 180 190 180 {
lab=vss}
N -340 -110 -340 20 {
lab=#net2}
C {devices/res.sym} 200 -80 0 0 {name=Ro_pt
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 70 -80 2 1 {name=Gm_pt value=1}
C {devices/res.sym} 70 50 0 0 {name=R1
value=1
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 70 140 0 0 {name=R2
value=1
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -420 -90 0 1 {name=Gma value=1}
C {devices/res.sym} -300 -90 2 0 {name=Ra
value=1
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} -380 -70 2 0 {name=p4 sig_type=std_logic lab=vfb
value=vref}
C {devices/lab_pin.sym} 30 90 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/vsource.sym} 170 -210 0 0 {name=V1 value=1 savecurrent=false}
C {devices/lab_pin.sym} 170 -180 3 0 {name=p1 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -380 -50 3 0 {name=p3 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 70 190 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 330 0 2 0 {name=p2 sig_type=std_logic lab=vout}
C {devices/res.sym} 290 100 0 0 {name=Rl
value=1
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 170 -290 2 0 {name=p7 sig_type=std_logic lab=vdd}
C {capa.sym} -10 -130 3 0 {name=Cgg_pt
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -10 -30 3 0 {name=Cgd_pt
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -230 -90 0 0 {name=Ca
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 190 100 0 0 {name=Cl
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {devices/vsource.sym} -340 50 0 0 {name=V2 value=1 savecurrent=false}
C {devices/lab_pin.sym} -340 80 3 0 {name=p8 sig_type=std_logic lab=vss}
