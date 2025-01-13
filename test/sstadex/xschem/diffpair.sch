v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -250 -130 -250 -110 {
lab=out_pos}
N -250 -50 -250 -30 {
lab=vs}
N -120 -130 -120 -110 {
lab=out_pos}
N -180 -130 -120 -130 {
lab=out_pos}
N -120 -50 -120 -30 {
lab=vs}
N -180 -30 -120 -30 {
lab=vs}
N 110 -130 110 -110 {
lab=out_pos}
N 110 -50 110 -30 {
lab=vs}
N 240 -130 240 -110 {
lab=out_pos}
N 180 -130 240 -130 {
lab=out_pos}
N 240 -50 240 -30 {
lab=vs}
N 180 -30 240 -30 {
lab=vs}
N -180 -30 -180 30 {
lab=vs}
N 0 30 180 30 {
lab=vs}
N 180 -30 180 30 {
lab=vs}
N 0 30 0 80 {
lab=vs}
N -180 -180 -180 -130 {
lab=out_pos}
N 180 -150 180 -130 {
lab=out_pos}
N -380 -100 -290 -100 {
lab=vneg}
N -290 -60 -290 -30 {
lab=vs}
N -290 -30 -250 -30 {
lab=vs}
N 70 -60 70 -30 {
lab=vs}
N 70 -30 110 -30 {
lab=vs}
N -30 -30 -30 -10 {
lab=vss}
N -30 -100 -30 -90 {
lab=vpos}
N -30 -100 70 -100 {
lab=vpos}
N -250 -30 -180 -30 {
lab=vs}
N 110 -30 180 -30 {
lab=vs}
N -180 30 0 30 {
lab=vs}
N -250 -130 -180 -130 {
lab=out_pos}
N 110 -130 180 -130 {
lab=out_pos}
N 180 -180 180 -150 {
lab=out_pos}
N 0 140 0 170 {
lab=vss}
N -380 -100 -380 -70 {
lab=vneg}
N -380 -10 -380 10 {
lab=vss}
C {devices/res.sym} -120 -80 0 0 {name=Rdif_2
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} -250 -80 0 0 {name=Gdif_2 value=10}
C {devices/res.sym} 240 -80 0 0 {name=Rdif_1
value=1000
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 110 -80 0 0 {name=Gdif_1 value=10}
C {devices/isource.sym} 0 110 0 0 {name=I2 value=1}
C {devices/lab_pin.sym} 0 30 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/vsource.sym} -30 -60 0 0 {name=V_p value=0.9 savecurrent=false}
C {devices/lab_pin.sym} -30 -100 1 0 {name=p3 sig_type=std_logic lab=vpos}
C {devices/lab_pin.sym} -30 -10 3 0 {name=p8 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 0 170 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/vsource.sym} -380 -40 0 0 {name=V_n value=0.9 savecurrent=false}
C {devices/lab_pin.sym} -380 -100 1 0 {name=p2 sig_type=std_logic lab=vneg}
C {devices/lab_pin.sym} -380 10 3 0 {name=p7 sig_type=std_logic lab=vss}
C {devices/lab_pin.sym} 180 -180 1 0 {name=p4 sig_type=std_logic lab=out_pos}
C {devices/lab_pin.sym} -180 -180 1 0 {name=p5 sig_type=std_logic lab=out_neg}
