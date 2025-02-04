v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -150 0 -150 60 {
lab=vs}
N 30 60 210 60 {
lab=vs}
N 30 60 30 110 {
lab=vs}
N 170 -280 210 -280 {
lab=vdd}
N -150 -150 -150 -100 {
lab=#net1}
N 210 -120 410 -120 {
lab=vout}
N -280 -60 -280 -30 {
lab=vn}
N -280 30 -280 50 {
lab=vn}
N -280 -60 -190 -60 {
lab=vn}
N 270 10 270 30 {
lab=vp1}
N 270 -60 270 -50 {
lab=vp1}
N 30 170 30 200 {
lab=vss}
N -150 60 30 60 {
lab=vs}
N -150 -280 -50 -280 {
lab=vdd}
N 210 -150 210 -120 {
lab=vout}
N -50 -280 170 -280 {
lab=vdd}
N -280 -30 -280 30 {
lab=vn}
N 270 -50 270 10 {
lab=vp1}
N 210 -180 210 -150 {
lab=vout}
N -150 -190 -150 -150 {
lab=#net1}
N -150 -280 -150 -250 {
lab=vdd}
N 210 -190 210 -180 {
lab=vout}
N 210 -280 210 -250 {
lab=vdd}
N 210 -120 210 -90 {
lab=vout}
N 210 -30 210 60 {
lab=vs}
N -150 -100 -150 -90 {
lab=#net1}
N -150 -30 -150 -0 {
lab=vs}
N 250 -60 270 -60 {
lab=vp1}
N -150 -60 210 -60 {
lab=vs}
N 50 -60 50 60 {
lab=vs}
N 30 200 30 260 {
lab=vss}
N -110 -220 170 -220 {
lab=#net1}
N 30 -220 30 -160 {
lab=#net1}
N -150 -160 30 -160 {
lab=#net1}
N -200 -220 -150 -220 {
lab=vdd}
N -200 -280 -200 -220 {
lab=vdd}
N -200 -280 -150 -280 {
lab=vdd}
N 210 -220 260 -220 {
lab=vdd}
N 260 -280 260 -220 {
lab=vdd}
N 210 -280 260 -280 {
lab=vdd}
N 380 -120 380 -110 {
lab=vout}
N 380 -110 380 -60 {
lab=vout}
N 380 10 380 220 {
lab=vss}
N 30 220 380 220 {
lab=vss}
N 380 -0 380 10 {
lab=vss}
C {devices/isource.sym} 30 140 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 30 60 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 410 -120 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 10 -280 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -280 50 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 270 30 3 0 {name=p8 sig_type=std_logic lab=vp1}
C {devices/lab_pin.sym} 30 260 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/code_shown.sym} -960 -270 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vref vn 0 0.9
Vdd vdd 0 1.8
Vpos vp1 0 0.9 ac 1
Vss vss 0 0

.control
ac dec 10 1 1G
plot vdb(vout)
plot phase(vout)
op 
print vout vs
.endc

"}
C {sky130_fd_pr/nfet_01v8_lvt.sym} 230 -60 0 1 {name=M1
W=20.9
L=1.6
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
C {sky130_fd_pr/nfet_01v8_lvt.sym} -170 -60 0 0 {name=M2
W=20.9
L=1.6
nf=1
mult=10
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=nfet_01v8_lvt
spiceprefix=X
}
C {sky130_fd_pr/pfet_01v8_lvt.sym} 190 -220 0 0 {name=M3
W=1.1
L=0.4
nf=1
mult=1
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=pfet_01v8_lvt
spiceprefix=X
}
C {sky130_fd_pr/pfet_01v8_lvt.sym} -130 -220 0 1 {name=M4
W=1.1
L=0.4
nf=1
mult=1
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=pfet_01v8_lvt
spiceprefix=X
}
C {capa.sym} 380 -30 0 0 {name=C5
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
