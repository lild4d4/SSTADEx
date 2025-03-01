v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -300 -20 -300 0 {
lab=#net1}
N -300 60 -300 80 {
lab=vs}
N 60 -20 60 0 {
lab=#net2}
N 60 60 60 80 {
lab=vs}
N 190 -20 190 0 {
lab=#net2}
N 130 -20 190 -20 {
lab=#net2}
N 190 60 190 80 {
lab=vs}
N 130 80 190 80 {
lab=vs}
N -230 80 -230 140 {
lab=vs}
N 130 80 130 140 {
lab=vs}
N -50 140 -50 190 {
lab=vs}
N 130 -550 130 -500 {
lab=vss}
N 90 -550 130 -550 {
lab=vss}
N -230 -550 -230 -500 {
lab=vss}
N -130 -550 -130 -500 {
lab=vss}
N 130 -550 230 -550 {
lab=vss}
N -430 10 -340 10 {
lab=vss}
N -340 50 -340 80 {
lab=vs}
N -340 80 -300 80 {
lab=vs}
N 20 50 20 80 {
lab=vs}
N 20 80 60 80 {
lab=vs}
N 90 -550 90 -490 {
lab=vss}
N -300 80 -230 80 {
lab=vs}
N 60 80 130 80 {
lab=vs}
N -230 140 -50 140 {
lab=vs}
N -300 -20 -230 -20 {
lab=#net1}
N 60 -20 130 -20 {
lab=#net2}
N -230 -550 -130 -550 {
lab=vss}
N 130 -440 130 -420 {
lab=#net3}
N 130 -70 130 -20 {
lab=#net2}
N 60 -170 60 -150 {
lab=vout}
N 60 -90 60 -70 {
lab=#net2}
N 190 -170 190 -150 {
lab=vout}
N 190 -90 190 -70 {
lab=#net2}
N 20 -70 60 -70 {
lab=#net2}
N 130 -70 190 -70 {
lab=#net2}
N 60 -70 130 -70 {
lab=#net2}
N 140 -170 190 -170 {
lab=vout}
N 140 -210 140 -170 {
lab=vout}
N 130 -420 230 -420 {
lab=#net3}
N 60 -170 140 -170 {
lab=vout}
N 140 -210 350 -210 {
lab=vout}
N 20 -100 20 -70 {
lab=#net2}
N -10 -140 20 -140 {
lab=vss}
N -10 -80 -10 -70 {
lab=#net2}
N -10 -70 20 -70 {
lab=#net2}
N 50 -170 60 -170 {
lab=vout}
N -10 -170 -10 -140 {
lab=vss}
N 230 -440 230 -420 {
lab=#net3}
N 230 -550 230 -500 {
lab=vss}
N 130 -280 130 -260 {
lab=vout}
N 140 -260 230 -260 {
lab=vout}
N 230 -280 230 -260 {
lab=vout}
N 230 -390 230 -340 {
lab=#net3}
N 140 -260 140 -210 {
lab=vout}
N 130 -260 140 -260 {
lab=vout}
N -230 -220 -230 -20 {
lab=#net1}
N 130 -390 230 -390 {
lab=#net3}
N 130 -390 130 -340 {
lab=#net3}
N 130 -420 130 -390 {
lab=#net3}
N -230 -440 -230 -420 {
lab=#net1}
N -230 -420 -130 -420 {
lab=#net1}
N -130 -440 -130 -420 {
lab=#net1}
N -230 -420 -230 -220 {
lab=#net1}
N 90 -360 90 -330 {
lab=#net3}
N 90 -390 130 -390 {
lab=#net3}
N 50 -290 90 -290 {
lab=#net4}
N -90 -550 -90 -490 {
lab=vss}
N -130 -550 -90 -550 {
lab=vss}
N -90 -450 -60 -450 {
lab=#net1}
N -230 -220 -60 -220 {
lab=#net1}
N 120 -260 130 -260 {
lab=vout}
N 50 -260 60 -260 {
lab=#net4}
N 50 -290 50 -260 {
lab=#net4}
N 110 -420 130 -420 {
lab=#net3}
N 20 -420 50 -420 {
lab=#net1}
N 20 -450 20 -420 {
lab=#net1}
N 50 -360 90 -360 {
lab=#net3}
N 90 -390 90 -360 {
lab=#net3}
N 50 -300 50 -290 {
lab=#net4}
N 20 -450 90 -450 {
lab=#net1}
N -90 -550 90 -550 {
lab=vss}
N -60 -450 20 -450 {
lab=#net1}
N -60 -450 -60 -220 {
lab=#net1}
N -50 270 -50 280 {
lab=vss}
N 350 -210 350 -180 {
lab=vout}
N -50 270 350 270 {
lab=vss}
N -50 250 -50 270 {
lab=vss}
N 350 -120 350 270 {
lab=vss}
N -80 10 20 10 {
lab=vpos}
N -50 140 130 140 {
lab=vs}
N -80 -140 -10 -140 {
lab=vss}
C {devices/vccs.sym} -300 30 0 0 {name=Gdif_m1_2 value=0.0001414}
C {devices/res.sym} 190 30 0 0 {name=Rdif_m1_1
value=1121489
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 60 30 0 0 {name=Gdif_m1_1 value=0.0001414}
C {devices/isource.sym} -50 220 0 0 {name=I2 value=0}
C {devices/lab_pin.sym} -50 140 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -80 10 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 320 -210 1 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -430 10 0 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -50 280 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/vccs.sym} 130 -470 2 1 {name=Gaload value=1.47e-5}
C {devices/vccs.sym} -130 -470 2 0 {name=Gaload_2 value=1.47e-5}
C {devices/res.sym} 190 -120 0 0 {name=Rdif_m2_1
value=1050237
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 60 -120 0 0 {name=Gdif_m2_1 value=6.97e-5}
C {devices/lab_pin.sym} -80 -140 3 0 {name=p10 sig_type=std_logic lab=vss}
C {capa.sym} -10 -110 0 1 {name=Cgs_m2
m=1
value=9e-14
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 20 -170 1 1 {name=Cgd_m2
m=1
value=7.37e-16
footprint=1206
device="ceramic capacitor"}
C {devices/code_shown.sym} -990 -390 0 0 {name=s1 only_toplevel=false value="

Vdd vdd 0 0
Vpos vpos 0 0 ac 1
Vbias vbias 0 0
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI
meas ac gain find vdb(vout) at=1
let low_gain = gain-3
meas ac fc WHEN vdb(vout)=low_gain

let phas = ph(vout)*180/PI

meas ac fu WHEN vdb(vout)=0
meas ac pmar find phas at=fu
.endc

"}
C {devices/lab_pin.sym} -60 -550 1 0 {name=p2 sig_type=std_logic lab=vss}
C {devices/vccs.sym} 130 -310 2 1 {name=Gaload1 value=2.5e-4}
C {devices/lab_pin.sym} 20 -290 1 0 {name=p5 sig_type=std_logic lab=vss}
C {capa.sym} 80 -420 3 1 {name=Cgd_m1
m=1
value=5.74e-14
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 90 -260 3 1 {name=Cgd_m3
m=1
value=3e-15
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 50 -330 0 1 {name=Cgs_m1
m=1
value=3.2e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 350 -150 0 0 {name=Cds_m5
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
