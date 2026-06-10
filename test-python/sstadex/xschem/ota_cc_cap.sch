v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -300 -40 -300 -20 {
lab=#net1}
N -300 40 -300 60 {
lab=vs}
N -170 -40 -170 -20 {
lab=#net1}
N -230 -40 -170 -40 {
lab=#net1}
N -170 40 -170 60 {
lab=vs}
N -230 60 -170 60 {
lab=vs}
N 60 -40 60 -20 {
lab=#net2}
N 60 40 60 60 {
lab=vs}
N 190 -40 190 -20 {
lab=#net2}
N 130 -40 190 -40 {
lab=#net2}
N 190 40 190 60 {
lab=vs}
N 130 60 190 60 {
lab=vs}
N -230 60 -230 120 {
lab=vs}
N -50 120 130 120 {
lab=vs}
N 130 60 130 120 {
lab=vs}
N -50 120 -50 170 {
lab=vs}
N 130 -390 130 -340 {
lab=vdd}
N 90 -390 130 -390 {
lab=vdd}
N -230 -390 -230 -340 {
lab=vdd}
N -230 -90 -230 -40 {
lab=#net1}
N -130 -390 -130 -340 {
lab=vdd}
N -130 -280 -130 -260 {
lab=#net3}
N -230 -260 -130 -260 {
lab=#net3}
N 230 -390 230 -340 {
lab=vdd}
N 230 -280 230 -260 {
lab=vout}
N 140 -260 230 -260 {
lab=vout}
N 130 -390 230 -390 {
lab=vdd}
N -430 -10 -430 20 {
lab=vneg}
N -430 80 -430 100 {
lab=vss}
N -430 -10 -340 -10 {
lab=vneg}
N -340 30 -340 60 {
lab=vs}
N -340 60 -300 60 {
lab=vs}
N 20 30 20 60 {
lab=vs}
N 20 60 60 60 {
lab=vs}
N -80 60 -80 80 {
lab=vss}
N -80 -10 -80 0 {
lab=vpos}
N -270 -390 -270 -330 {
lab=vdd}
N -270 -390 -230 -390 {
lab=vdd}
N 90 -390 90 -330 {
lab=vdd}
N -270 -260 -230 -260 {
lab=#net3}
N -270 -290 -270 -260 {
lab=#net3}
N -130 -260 90 -260 {
lab=#net3}
N 90 -290 90 -260 {
lab=#net3}
N -300 60 -230 60 {
lab=vs}
N 60 60 130 60 {
lab=vs}
N -230 120 -50 120 {
lab=vs}
N -300 -40 -230 -40 {
lab=#net1}
N 60 -40 130 -40 {
lab=#net2}
N -230 -390 -130 -390 {
lab=vdd}
N -230 -280 -230 -260 {
lab=#net3}
N 130 -280 130 -260 {
lab=vout}
N 130 -90 130 -40 {
lab=#net2}
N -130 -390 90 -390 {
lab=vdd}
N 250 -40 250 -20 {
lab=#net2}
N 190 -40 250 -40 {
lab=#net2}
N 250 40 250 60 {
lab=vs}
N 190 60 250 60 {
lab=vs}
N -80 -10 20 -10 {
lab=vpos}
N -50 230 -50 260 {
lab=vss}
N 60 -190 60 -170 {
lab=vout}
N 60 -110 60 -90 {
lab=#net2}
N 190 -190 190 -170 {
lab=vout}
N 190 -110 190 -90 {
lab=#net2}
N 20 -90 60 -90 {
lab=#net2}
N 130 -90 190 -90 {
lab=#net2}
N 250 -190 250 -170 {
lab=vout}
N 250 -110 250 -90 {
lab=#net2}
N 190 -90 250 -90 {
lab=#net2}
N 60 -90 130 -90 {
lab=#net2}
N 190 -190 250 -190 {
lab=vout}
N 140 -190 190 -190 {
lab=vout}
N 140 -230 140 -190 {
lab=vout}
N 130 -260 140 -260 {
lab=vout}
N 60 -190 140 -190 {
lab=vout}
N 140 -230 320 -230 {
lab=vout}
N 140 -260 140 -230 {
lab=vout}
N -300 -190 -300 -170 {
lab=#net3}
N -300 -110 -300 -90 {
lab=#net1}
N -170 -190 -170 -170 {
lab=#net3}
N -170 -110 -170 -90 {
lab=#net1}
N -230 -90 -170 -90 {
lab=#net1}
N -300 -90 -230 -90 {
lab=#net1}
N -230 -190 -170 -190 {
lab=#net3}
N -230 -260 -230 -190 {
lab=#net3}
N -300 -190 -230 -190 {
lab=#net3}
N 20 -120 20 -90 {
lab=#net2}
N -130 -120 -130 -90 {
lab=#net1}
N -170 -90 -130 -90 {
lab=#net1}
N -10 -160 20 -160 {
lab=vbias}
N -60 -220 -60 -160 {
lab=vbias}
N -130 -160 -60 -160 {
lab=vbias}
N -140 -220 -120 -220 {
lab=vss}
N -10 -100 -10 -90 {
lab=#net2}
N -10 -90 20 -90 {
lab=#net2}
N 50 -190 60 -190 {
lab=vout}
N -10 -190 -10 -160 {
lab=vbias}
N -60 -160 -10 -160 {
lab=vbias}
C {devices/res.sym} -170 10 0 0 {name=Rdif_m1_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -300 10 0 0 {name=Gdif_m1_2 value=10}
C {devices/res.sym} 190 10 0 0 {name=Rdif_m1_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 60 10 0 0 {name=Gdif_m1_1 value=10}
C {devices/isource.sym} -50 200 0 0 {name=I2 value=1}
C {devices/vsource.sym} 0 -360 0 0 {name=V1 value=1.8 savecurrent=false}
C {devices/vsource.sym} -430 50 0 0 {name=V_n value=0.9 savecurrent=false}
C {devices/lab_pin.sym} -50 120 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -430 -10 1 0 {name=p2 sig_type=std_logic lab=vneg}
C {devices/vsource.sym} -80 30 0 0 {name=V_p value=0.9 savecurrent=false}
C {devices/lab_pin.sym} -80 -10 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 320 -230 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -60 -390 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 0 -330 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -430 100 3 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -80 80 3 0 {name=p8 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -50 260 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 230 -310 0 0 {name=Raload_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} -130 -310 0 0 {name=Raload_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 130 -310 2 1 {name=Gaload value=0.0002236}
C {devices/vccs.sym} -230 -310 2 1 {name=Gaload_2 value=0.0002236}
C {devices/res.sym} 190 -140 0 0 {name=Rdif_m2_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 60 -140 0 0 {name=Gdif_m2_1 value=10}
C {devices/res.sym} -300 -140 0 0 {name=Rdif_m2_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -170 -140 0 1 {name=Gdif_m2_2 value=10}
C {devices/lab_pin.sym} -80 -160 3 0 {name=p10 sig_type=std_logic lab=vbias}
C {devices/vsource.sym} -90 -220 1 0 {name=V_b value=0.9 savecurrent=false}
C {devices/lab_pin.sym} -140 -220 0 0 {name=p11 sig_type=std_logic lab=vss}
C {capa.sym} 250 10 0 0 {name=Cds_m1
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 250 -140 0 0 {name=Cds_m2
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -10 -130 0 1 {name=Cgs_m2
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 20 -190 1 1 {name=Cgd_m2
m=1
value=1
footprint=1206
device="ceramic capacitor"}
