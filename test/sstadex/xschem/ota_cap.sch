v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -190 -60 -190 -40 {
lab=#net1}
N -190 20 -190 40 {
lab=vs}
N -60 -60 -60 -40 {
lab=#net1}
N -120 -60 -60 -60 {
lab=#net1}
N -60 20 -60 40 {
lab=vs}
N -120 40 -60 40 {
lab=vs}
N 170 -60 170 -40 {
lab=vout}
N 170 20 170 40 {
lab=vs}
N 300 -60 300 -40 {
lab=vout}
N 240 -60 300 -60 {
lab=vout}
N 300 20 300 40 {
lab=vs}
N 240 40 300 40 {
lab=vs}
N -120 40 -120 100 {
lab=vs}
N 60 100 240 100 {
lab=vs}
N 240 40 240 100 {
lab=vs}
N 60 100 60 150 {
lab=vs}
N 240 -240 240 -190 {
lab=vdd}
N 200 -240 240 -240 {
lab=vdd}
N -120 -240 -120 -190 {
lab=vdd}
N -120 -110 -120 -60 {
lab=#net1}
N 240 -80 240 -60 {
lab=vout}
N -20 -240 -20 -190 {
lab=vdd}
N -20 -130 -20 -110 {
lab=#net1}
N -120 -110 -20 -110 {
lab=#net1}
N 340 -240 340 -190 {
lab=vdd}
N 340 -130 340 -110 {
lab=vout}
N 240 -110 340 -110 {
lab=vout}
N 240 -240 340 -240 {
lab=vdd}
N 240 -80 440 -80 {
lab=vout}
N -320 -30 -320 0 {
lab=vneg}
N -320 60 -320 80 {
lab=vss}
N -320 -30 -230 -30 {
lab=vneg}
N -230 10 -230 40 {
lab=vs}
N -230 40 -190 40 {
lab=vs}
N 130 10 130 40 {
lab=vs}
N 130 40 170 40 {
lab=vs}
N 30 40 30 60 {
lab=vss}
N 30 -30 30 -20 {
lab=vpos}
N 60 210 60 240 {
lab=vss}
N 30 -30 130 -30 {
lab=vpos}
N -160 -240 -160 -180 {
lab=vdd}
N -160 -240 -120 -240 {
lab=vdd}
N 200 -240 200 -180 {
lab=vdd}
N -160 -110 -120 -110 {
lab=#net1}
N -160 -140 -160 -110 {
lab=#net1}
N -20 -110 200 -110 {
lab=#net1}
N 200 -140 200 -110 {
lab=#net1}
N -190 40 -120 40 {
lab=vs}
N 170 40 240 40 {
lab=vs}
N -120 100 60 100 {
lab=vs}
N -190 -60 -120 -60 {
lab=#net1}
N 170 -60 240 -60 {
lab=vout}
N -120 -240 -20 -240 {
lab=vdd}
N -120 -130 -120 -110 {
lab=#net1}
N 240 -130 240 -110 {
lab=vout}
N 240 -110 240 -80 {
lab=vout}
N -20 -240 200 -240 {
lab=vdd}
N 80 40 80 60 {
lab=vs}
N 80 -30 80 -20 {
lab=vpos}
N 80 60 130 60 {
lab=vs}
N 130 40 130 60 {
lab=vs}
N 150 -60 170 -60 {
lab=vout}
N 80 -60 90 -60 {
lab=vpos}
N 80 -60 80 -30 {
lab=vpos}
N 430 -80 430 -40 {
lab=vout}
N 430 20 430 230 {
lab=vss}
N 60 230 430 230 {
lab=vss}
N 360 -60 360 -40 {
lab=vout}
N 300 -60 360 -60 {
lab=vout}
N 360 20 360 40 {
lab=vs}
N 300 40 360 40 {
lab=vs}
C {devices/res.sym} -60 -10 0 0 {name=Rdif_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -190 -10 0 0 {name=Gdif_2 value=10}
C {devices/res.sym} 300 -10 0 0 {name=Rdif_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 170 -10 0 0 {name=Gdif_1 value=10}
C {devices/isource.sym} 60 180 0 0 {name=I2 value=1}
C {devices/vsource.sym} 110 -210 0 0 {name=V1 value=1.8 savecurrent=false}
C {devices/vsource.sym} -320 30 0 0 {name=V_n value=0.9 savecurrent=false}
C {devices/lab_pin.sym} 60 100 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -320 -30 1 0 {name=p2 sig_type=std_logic lab=vneg}
C {devices/vsource.sym} 30 10 0 0 {name=V_p value=0.9 savecurrent=false}
C {devices/lab_pin.sym} 30 -30 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 440 -80 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 50 -240 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 110 -180 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -320 80 3 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 30 60 3 0 {name=p8 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 60 240 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 340 -160 0 0 {name=Raload_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} -20 -160 0 0 {name=Raload_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 240 -160 2 1 {name=Gaload value=0.0002236}
C {capa.sym} 120 -60 3 0 {name=Cgd
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 80 10 0 0 {name=Cgs
m=1
value=1
footprint=1206
device="ceramic capacitor"}
