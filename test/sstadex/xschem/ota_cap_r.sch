v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -330 -60 -330 -40 {
lab=#net1}
N -330 20 -330 40 {
lab=vs}
N -200 -60 -200 -40 {
lab=#net1}
N -260 -60 -200 -60 {
lab=#net1}
N -200 20 -200 40 {
lab=vs}
N -260 40 -200 40 {
lab=vs}
N 30 -60 30 -40 {
lab=vout}
N 30 20 30 40 {
lab=vs}
N 160 -60 160 -40 {
lab=vout}
N 100 -60 160 -60 {
lab=vout}
N 160 20 160 40 {
lab=vs}
N 100 40 160 40 {
lab=vs}
N -260 40 -260 100 {
lab=vs}
N -80 100 100 100 {
lab=vs}
N 100 40 100 100 {
lab=vs}
N -80 100 -80 150 {
lab=vs}
N 100 -240 100 -190 {
lab=vdd}
N 60 -240 100 -240 {
lab=vdd}
N -260 -240 -260 -190 {
lab=vdd}
N -260 -110 -260 -60 {
lab=#net1}
N 100 -80 100 -60 {
lab=vout}
N -160 -240 -160 -190 {
lab=vdd}
N -160 -130 -160 -110 {
lab=#net1}
N -260 -110 -160 -110 {
lab=#net1}
N 200 -240 200 -190 {
lab=vdd}
N 200 -130 200 -110 {
lab=vout}
N 100 -110 200 -110 {
lab=vout}
N 100 -240 200 -240 {
lab=vdd}
N 100 -80 300 -80 {
lab=vout}
N -460 -30 -460 0 {
lab=vneg}
N -460 60 -460 80 {
lab=vss}
N -460 -30 -370 -30 {
lab=vneg}
N -370 10 -370 40 {
lab=vs}
N -370 40 -330 40 {
lab=vs}
N -10 10 -10 40 {
lab=vs}
N -10 40 30 40 {
lab=vs}
N -110 40 -110 60 {
lab=vss}
N -110 -30 -110 -20 {
lab=vpos}
N -80 210 -80 240 {
lab=vss}
N -110 -30 -10 -30 {
lab=vpos}
N -300 -240 -300 -180 {
lab=vdd}
N -300 -240 -260 -240 {
lab=vdd}
N 60 -240 60 -180 {
lab=vdd}
N -300 -110 -260 -110 {
lab=#net1}
N -300 -140 -300 -110 {
lab=#net1}
N -160 -110 60 -110 {
lab=#net1}
N 60 -140 60 -110 {
lab=#net1}
N -330 40 -260 40 {
lab=vs}
N 30 40 100 40 {
lab=vs}
N -260 100 -80 100 {
lab=vs}
N -330 -60 -260 -60 {
lab=#net1}
N 30 -60 100 -60 {
lab=vout}
N -260 -240 -160 -240 {
lab=vdd}
N -260 -130 -260 -110 {
lab=#net1}
N 100 -130 100 -110 {
lab=vout}
N 100 -110 100 -80 {
lab=vout}
N -160 -240 60 -240 {
lab=vdd}
N -60 40 -60 60 {
lab=vs}
N -60 -30 -60 -20 {
lab=vpos}
N -60 60 -10 60 {
lab=vs}
N -10 40 -10 60 {
lab=vs}
N 10 -60 30 -60 {
lab=vout}
N -60 -60 -50 -60 {
lab=vpos}
N -60 -60 -60 -30 {
lab=vpos}
N 290 -80 290 -40 {
lab=vout}
N -80 230 290 230 {
lab=vss}
N 220 -60 220 -40 {
lab=vout}
N 160 -60 220 -60 {
lab=vout}
N 220 20 220 40 {
lab=vs}
N 160 40 220 40 {
lab=vs}
N 290 150 290 230 {
lab=vss}
N 290 40 290 90 {
lab=#net2}
N 290 -40 290 -30 {
lab=vout}
N 290 30 290 40 {
lab=#net2}
C {devices/res.sym} -200 -10 0 0 {name=Rdif_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -330 -10 0 0 {name=Gdif_2 value=10}
C {devices/res.sym} 160 -10 0 0 {name=Rdif_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 30 -10 0 0 {name=Gdif_1 value=10}
C {devices/isource.sym} -80 180 0 0 {name=I2 value=1}
C {devices/vsource.sym} -30 -210 0 0 {name=V1 value=1.8 savecurrent=false}
C {devices/vsource.sym} -460 30 0 0 {name=V_n value=0.9 savecurrent=false}
C {devices/lab_pin.sym} -80 100 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} -460 -30 1 0 {name=p2 sig_type=std_logic lab=vneg}
C {devices/vsource.sym} -110 10 0 0 {name=V_p value=0.9 savecurrent=false}
C {devices/lab_pin.sym} -110 -30 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} 300 -80 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -90 -240 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -30 -180 3 0 {name=p6 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -460 80 3 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -110 60 3 0 {name=p8 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} -80 240 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/res.sym} 200 -160 0 0 {name=Raload_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} -160 -160 0 0 {name=Raload_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 100 -160 2 1 {name=Gaload value=0.0002236}
C {capa.sym} -20 -60 3 0 {name=Cgd
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {capa.sym} -60 10 0 0 {name=Cgs
m=1
value=1
footprint=1206
device="ceramic capacitor"}
C {devices/vsource.sym} 290 120 0 0 {name=Vr value=0.9 savecurrent=false}
C {devices/res.sym} 290 0 0 0 {name=Rr
value=1000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 290 60 2 0 {name=p10 sig_type=std_logic lab=vr}
