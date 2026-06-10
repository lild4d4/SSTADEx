v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -130 -20 -130 40 {
lab=vs}
N 50 40 230 40 {
lab=vs}
N 50 40 50 90 {
lab=vs}
N 190 -300 230 -300 {
lab=vdd}
N -130 -170 -130 -120 {
lab=#net1}
N 230 -140 430 -140 {
lab=vout}
N -260 -80 -260 -50 {
lab=vn}
N -260 10 -260 30 {
lab=vn}
N -260 -80 -170 -80 {
lab=vn}
N 290 -10 290 10 {
lab=vp}
N 290 -80 290 -70 {
lab=vp}
N 50 150 50 180 {
lab=vss}
N -130 40 50 40 {
lab=vs}
N -130 -300 -30 -300 {
lab=vdd}
N 230 -170 230 -140 {
lab=vout}
N -30 -300 190 -300 {
lab=vdd}
N -260 -50 -260 10 {
lab=vn}
N 290 -70 290 -10 {
lab=vp}
N 230 -200 230 -170 {
lab=vout}
N -130 -210 -130 -170 {
lab=#net1}
N -130 -300 -130 -270 {
lab=vdd}
N 230 -210 230 -200 {
lab=vout}
N 230 -300 230 -270 {
lab=vdd}
N 230 -140 230 -110 {
lab=vout}
N 230 -50 230 40 {
lab=vs}
N -130 -120 -130 -110 {
lab=#net1}
N -130 -50 -130 -20 {
lab=vs}
N 270 -80 290 -80 {
lab=vp}
N -130 -80 230 -80 {
lab=vs}
N 70 -80 70 40 {
lab=vs}
N 50 180 50 240 {
lab=vss}
N -90 -240 190 -240 {
lab=#net1}
N 50 -240 50 -180 {
lab=#net1}
N -130 -180 50 -180 {
lab=#net1}
N -180 -240 -130 -240 {
lab=vdd}
N -180 -300 -180 -240 {
lab=vdd}
N -180 -300 -130 -300 {
lab=vdd}
N 230 -240 280 -240 {
lab=vdd}
N 280 -300 280 -240 {
lab=vdd}
N 230 -300 280 -300 {
lab=vdd}
C {devices/isource.sym} 50 120 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 50 40 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 430 -140 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 30 -300 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -260 30 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 290 10 3 0 {name=p8 sig_type=std_logic lab=vp}
C {devices/lab_pin.sym} 50 240 3 0 {name=p9 sig_type=std_logic lab=vss}
C {devices/code_shown.sym} -320 160 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vref vn 0 0.9
Vdd vdd 0 1.8
Vpos vp 0 0.9 ac 1
Vss vss 0 0

.control
ac dec 10 1 10000G
plot vdb(vout) 
.endc

.control
tran 10n 1u
plot vout vs
.endc

"}
C {sky130_fd_pr/nfet_01v8_lvt.sym} 250 -80 0 1 {name=M1
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
C {sky130_fd_pr/nfet_01v8_lvt.sym} -150 -80 0 0 {name=M2
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
spiceprefix=X
}
C {sky130_fd_pr/pfet_01v8_lvt.sym} 210 -240 0 0 {name=M3
W=27.28
L=6.4
nf=1
mult=10
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=pfet_01v8_lvt
spiceprefix=X
}
C {sky130_fd_pr/pfet_01v8_lvt.sym} -110 -240 0 1 {name=M4
W=27.28
L=6.4
nf=1
mult=10
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=pfet_01v8_lvt
spiceprefix=X
}
