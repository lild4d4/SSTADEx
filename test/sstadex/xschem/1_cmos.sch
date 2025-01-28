v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N 320 240 320 280 {
lab=vss}
N 320 140 320 180 {
lab=vout}
N 320 160 400 160 {
lab=vout}
N 50 210 100 210 {
lab=vm}
N 100 210 130 210 {
lab=vm}
N 130 210 160 210 {
lab=vm}
N 50 280 50 290 {
lab=vss}
N 50 290 50 310 {
lab=vss}
N 50 290 170 290 {
lab=vss}
N 170 280 170 290 {
lab=vss}
N 170 210 170 220 {
lab=vm}
N 50 210 50 220 {
lab=vm}
N 160 210 210 210 {
lab=vm}
N 130 230 130 270 {
lab=vn}
N 90 230 130 230 {
lab=vn}
N 130 270 130 310 {
lab=vn}
N 210 210 280 210 {
lab=vm}
N 320 280 320 310 {
lab=vss}
N 50 -230 100 -230 {
lab=#net1}
N 100 -230 130 -230 {
lab=#net1}
N 130 -230 160 -230 {
lab=#net1}
N 50 -160 50 -150 {
lab=vss}
N 50 -150 50 -130 {
lab=vss}
N 50 -150 170 -150 {
lab=vss}
N 170 -160 170 -150 {
lab=vss}
N 170 -230 170 -220 {
lab=#net1}
N 50 -230 50 -220 {
lab=#net1}
N 130 -210 130 -170 {
lab=vss}
N 90 -210 130 -210 {
lab=vss}
N 130 -170 130 -130 {
lab=vss}
N 320 -260 320 -240 {
lab=vout2}
N 320 -180 320 -160 {
lab=vss}
N 450 -260 450 -250 {
lab=vout2}
N 320 -260 450 -260 {
lab=vout2}
N 450 -250 450 -240 {
lab=vout2}
N 450 -180 450 -160 {
lab=vss}
N 320 -160 450 -160 {
lab=vss}
N 320 -160 320 -110 {
lab=vss}
N 160 -230 170 -230 {
lab=#net1}
N 320 -290 400 -290 {
lab=vout2}
N 320 -110 320 -80 {
lab=vss}
N 450 -260 510 -260 {
lab=vout2}
N 510 -260 520 -260 {
lab=vout2}
N 520 -260 520 -240 {
lab=vout2}
N 450 -160 520 -160 {
lab=vss}
N 520 -180 520 -160 {
lab=vss}
N 320 210 350 210 {
lab=vss}
N 350 210 350 260 {
lab=vss}
N 320 260 350 260 {
lab=vss}
N 320 60 320 80 {
lab=vdd}
N 320 -390 320 -370 {
lab=vss}
N 320 -310 320 -260 {
lab=vout2}
N 170 -230 280 -230 {
lab=#net1}
N 280 -190 280 -160 {
lab=vss}
N 280 -160 320 -160 {
lab=vss}
N 280 -260 320 -260 {
lab=vout2}
N 200 -260 220 -260 {
lab=#net1}
N 200 -260 200 -230 {
lab=#net1}
N 250 -230 250 -220 {
lab=#net1}
N 250 -160 280 -160 {
lab=vss}
N 430 130 430 160 {
lab=vout}
N 400 160 430 160 {
lab=vout}
N 430 50 430 70 {
lab=vdd}
N 320 50 430 50 {
lab=vdd}
N 320 50 320 60 {
lab=vdd}
N 420 -400 420 -380 {
lab=vss}
N 320 -400 420 -400 {
lab=vss}
N 320 -400 320 -390 {
lab=vss}
N 420 -320 420 -290 {
lab=vout2}
N 400 -290 420 -290 {
lab=vout2}
N 470 160 470 220 {
lab=vout}
N 430 160 470 160 {
lab=vout}
N 470 280 470 310 {
lab=vss}
N 320 310 470 310 {
lab=vss}
N 270 160 320 160 {
lab=vout}
N 190 160 210 160 {
lab=vm}
N 190 160 190 210 {
lab=vm}
C {devices/isource.sym} 320 110 0 0 {name=I1 value=40e-6}
C {devices/lab_pin.sym} 320 60 1 0 {name=p5 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} 400 160 2 0 {name=p6 sig_type=std_logic lab=vout}
C {devices/vccs.sym} 50 250 2 0 {name=Gma value=0.0004627}
C {devices/res.sym} 170 250 2 0 {name=Ra
value=55104
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 90 270 2 0 {name=p7 sig_type=std_logic lab=vp}
C {devices/lab_pin.sym} 50 310 3 0 {name=p8 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 130 310 2 0 {name=p9 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 320 310 3 0 {name=p10 sig_type=std_logic lab=vss
value=vref}
C {devices/code_shown.sym} 770 80 0 0 {name=s2 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vref vn 0 0.9
Vdd vdd 0 1.8
Vpos vp 0 0.9 ac 1
Vss vss 0 0

.control
ac dec 10 1 10000G
plot vdb(vout) vdb(vout2)
plot phase(vout) phase(vout2)
.endc


"}
C {devices/vccs.sym} 50 -190 2 0 {name=Gma1 value=0.0004627}
C {devices/res.sym} 170 -190 2 0 {name=Ra1
value=55104
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 90 -170 2 0 {name=p3 sig_type=std_logic lab=vp}
C {devices/lab_pin.sym} 50 -130 3 0 {name=p1 sig_type=std_logic lab=vss
value=vref}
C {devices/res.sym} 450 -210 0 0 {name=Ro_pt
value=4.91151e10
footprint=1206
device=resistor
m=1}
C {devices/vccs.sym} 320 -210 0 0 {name=Gm_pt value=3.3e-7}
C {devices/isource.sym} 320 -340 0 0 {name=I2 value=0}
C {devices/lab_pin.sym} 400 -290 2 0 {name=p4 sig_type=std_logic lab=vout2}
C {devices/lab_pin.sym} 320 -80 3 0 {name=p11 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 130 -130 3 0 {name=p13 sig_type=std_logic lab=vss
value=vref}
C {devices/lab_pin.sym} 320 -390 1 0 {name=p2 sig_type=std_logic lab=vss
value=vref}
C {sky130_fd_pr/nfet_01v8_lvt.sym} 300 210 0 0 {name=M1
W=29.36
L=6.4
nf=1
mult=10
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=nfet_01v8_lvt
spiceprefix=X}
C {devices/lab_pin.sym} 240 210 3 0 {name=p12 sig_type=std_logic lab=vm
value=vref}
C {capa.sym} 470 250 0 0 {name=C3
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {capa.sym} 520 -210 0 0 {name=C4
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
