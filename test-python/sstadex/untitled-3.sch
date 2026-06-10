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
N -170 -20 -170 0 {
lab=#net1}
N -230 -20 -170 -20 {
lab=#net1}
N -170 60 -170 80 {
lab=vs}
N -230 80 -170 80 {
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
N -50 140 130 140 {
lab=vs}
N 130 80 130 140 {
lab=vs}
N -50 140 -50 190 {
lab=vs}
N 130 -370 130 -320 {
lab=vdd}
N 90 -370 130 -370 {
lab=vdd}
N -230 -370 -230 -320 {
lab=vdd}
N -230 -70 -230 -20 {
lab=#net1}
N -130 -370 -130 -320 {
lab=vdd}
N -130 -260 -130 -240 {
lab=#net3}
N -230 -240 -130 -240 {
lab=#net3}
N 230 -370 230 -320 {
lab=vdd}
N 230 -260 230 -240 {
lab=vout}
N 140 -240 230 -240 {
lab=vout}
N 130 -370 230 -370 {
lab=vdd}
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
N -270 -370 -270 -310 {
lab=vdd}
N -270 -370 -230 -370 {
lab=vdd}
N 90 -370 90 -310 {
lab=vdd}
N -270 -240 -230 -240 {
lab=#net3}
N -270 -270 -270 -240 {
lab=#net3}
N -130 -240 90 -240 {
lab=#net3}
N 90 -270 90 -240 {
lab=#net3}
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
N -230 -370 -130 -370 {
lab=vdd}
N -230 -260 -230 -240 {
lab=#net3}
N 130 -260 130 -240 {
lab=vout}
N 130 -70 130 -20 {
lab=#net2}
N -130 -370 90 -370 {
lab=vdd}
N 250 -20 250 0 {
lab=#net2}
N 190 -20 250 -20 {
lab=#net2}
N 250 60 250 80 {
lab=vs}
N 190 80 250 80 {
lab=vs}
N -80 10 20 10 {
lab=vpos}
N -50 250 -50 280 {
lab=vss}
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
N 250 -170 250 -150 {
lab=vout}
N 250 -90 250 -70 {
lab=#net2}
N 190 -70 250 -70 {
lab=#net2}
N 60 -70 130 -70 {
lab=#net2}
N 190 -170 250 -170 {
lab=vout}
N 140 -170 190 -170 {
lab=vout}
N 140 -210 140 -170 {
lab=vout}
N 130 -240 140 -240 {
lab=vout}
N 60 -170 140 -170 {
lab=vout}
N 140 -210 320 -210 {
lab=vout}
N 140 -240 140 -210 {
lab=vout}
N -300 -170 -300 -150 {
lab=#net3}
N -300 -90 -300 -70 {
lab=#net1}
N -170 -170 -170 -150 {
lab=#net3}
N -170 -90 -170 -70 {
lab=#net1}
N -230 -70 -170 -70 {
lab=#net1}
N -300 -70 -230 -70 {
lab=#net1}
N -230 -170 -170 -170 {
lab=#net3}
N -230 -240 -230 -170 {
lab=#net3}
N -300 -170 -230 -170 {
lab=#net3}
N 20 -100 20 -70 {
lab=#net2}
N -130 -100 -130 -70 {
lab=#net1}
N -170 -70 -130 -70 {
lab=#net1}
N -10 -140 20 -140 {
lab=vbias}
N -130 -140 -10 -140 {
lab=vbias}
N -10 -80 -10 -70 {
lab=#net2}
N -10 -70 20 -70 {
lab=#net2}
N 50 -170 60 -170 {
lab=vout}
N -10 -170 -10 -140 {
lab=vbias}
C {devices/res.sym} -170 30 0 0 {name=Rdif_m1_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -300 30 0 0 {name=Gdif_m1_2 value=10}
C {devices/res.sym} 190 30 0 0 {name=Rdif_m1_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 60 30 0 0 {name=Gdif_m1_1 value=10}
C {devices/isource.sym} -50 220 0 0 {name=I2 value=1}
C {devices/vsource.sym} 0 -340 0 0 {name=V1 value=1.8 savecurrent=false}
C {devices/lab_pin.sym} -50 140 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -80 10 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 320 -210 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -60 -370 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 0 -310 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -430 10 0 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -50 280 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 230 -290 0 0 {name=Raload_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} -130 -290 0 0 {name=Raload_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 130 -290 2 1 {name=Gaload value=0.0002236}
C {devices/vccs.sym} -230 -290 2 1 {name=Gaload_2 value=0.0002236}
C {devices/res.sym} 190 -120 0 0 {name=Rdif_m2_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 60 -120 0 0 {name=Gdif_m2_1 value=10}
C {devices/res.sym} -300 -120 0 0 {name=Rdif_m2_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -170 -120 0 1 {name=Gdif_m2_2 value=10}
C {devices/lab_pin.sym} -80 -140 3 0 {name=p10 sig_type=std_logic lab=vbias}
C {capa.sym} 250 30 0 0 {name=Cds_m1
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 250 -120 0 0 {name=Cds_m2
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -10 -110 0 1 {name=Cgs_m2
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 20 -170 1 1 {name=Cgd_m2
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {devices/code_shown.sym} -990 -380 0 0 {name=s1 only_toplevel=false value="

Vin vin 0 1 ac 1
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)*180/PI
meas ac gain find vdb(vout) at=1
let low_gain = gain-3

meas ac fc WHEN vdb(vout)=low_gain
print fc/(2*PI)
.endc

"}
