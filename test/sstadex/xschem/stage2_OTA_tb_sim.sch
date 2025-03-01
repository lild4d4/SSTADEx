v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -180 40 -180 100 {
lab=vs}
N 0 100 180 100 {
lab=vs}
N 0 100 0 150 {
lab=vs}
N 140 -240 180 -240 {
lab=vdd}
N -180 -110 -180 -60 {
lab=#net1}
N 180 -80 380 -80 {
lab=vout_s1}
N -310 -20 -310 10 {
lab=vn}
N -310 70 -310 90 {
lab=vn}
N -310 -20 -220 -20 {
lab=vn}
N 240 50 240 70 {
lab=vp1}
N 240 -20 240 -10 {
lab=vp1}
N 0 210 0 240 {
lab=vss}
N -180 100 0 100 {
lab=vs}
N -180 -240 -80 -240 {
lab=vdd}
N 180 -110 180 -80 {
lab=vout_s1}
N -80 -240 140 -240 {
lab=vdd}
N -310 10 -310 70 {
lab=vn}
N 240 -10 240 50 {
lab=vp1}
N 180 -140 180 -110 {
lab=vout_s1}
N -180 -150 -180 -110 {
lab=#net1}
N -180 -240 -180 -210 {
lab=vdd}
N 180 -150 180 -140 {
lab=vout_s1}
N 180 -240 180 -210 {
lab=vdd}
N 180 -80 180 -50 {
lab=vout_s1}
N 180 10 180 100 {
lab=vs}
N -180 -60 -180 -50 {
lab=#net1}
N -180 10 -180 40 {
lab=vs}
N 220 -20 240 -20 {
lab=vp1}
N -180 -20 180 -20 {
lab=vs}
N 20 -20 20 100 {
lab=vs}
N 0 240 0 300 {
lab=vss}
N -140 -180 140 -180 {
lab=#net1}
N 0 -180 0 -120 {
lab=#net1}
N -180 -120 0 -120 {
lab=#net1}
N -230 -180 -180 -180 {
lab=vdd}
N -230 -240 -230 -180 {
lab=vdd}
N -230 -240 -180 -240 {
lab=vdd}
N 180 -180 230 -180 {
lab=vdd}
N 230 -240 230 -180 {
lab=vdd}
N 180 -240 230 -240 {
lab=vdd}
N 420 -240 420 -110 {
lab=vdd}
N 230 -240 420 -240 {
lab=vdd}
N 420 -80 450 -80 {
lab=vdd}
N 450 -130 450 -80 {
lab=vdd}
N 420 -130 450 -130 {
lab=vdd}
N 420 -20 470 -20 {
lab=vout}
N 420 -50 420 -20 {
lab=vout}
N 420 -20 420 50 {
lab=vout}
N 420 120 420 300 {
lab=vss}
N 0 300 420 300 {
lab=vss}
N 420 110 420 120 {
lab=vss}
N 470 -20 600 -20 {
lab=vout}
N 550 -20 550 90 {
lab=vout}
N 550 150 550 300 {
lab=vss}
N 420 300 550 300 {
lab=vss}
N 670 -20 670 100 {
lab=vout}
N 600 -20 670 -20 {
lab=vout}
N 670 160 670 300 {
lab=vss}
N 550 300 670 300 {
lab=vss}
C {devices/isource.sym} 0 180 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 0 100 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 600 -20 1 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -20 -240 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -310 90 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 240 70 3 0 {name=p8 sig_type=std_logic lab=vp1}
C {devices/lab_pin.sym} 0 300 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sky130_fd_pr/nfet_01v8_lvt.sym} 200 -20 0 1 {name=M1
W=7.15
L=0.8
nf=1
mult=1
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=nfet_01v8_lvt
spiceprefix=X}
C {sky130_fd_pr/nfet_01v8_lvt.sym} -200 -20 0 0 {name=M2
W=7.15
L=0.8
nf=1
mult=1
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=nfet_01v8_lvt
spiceprefix=X
}
C {sky130_fd_pr/pfet_01v8_lvt.sym} 160 -180 0 0 {name=M3
W=10.8
L=1.6
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
C {sky130_fd_pr/pfet_01v8_lvt.sym} -160 -180 0 1 {name=M4
W=10.8
L=1.6
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
C {sky130_fd_pr/pfet_01v8_lvt.sym} 400 -80 0 0 {name=M5
W=1.454
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
C {devices/isource.sym} 420 80 0 0 {name=I1 value=20e-6}
C {devices/lab_pin.sym} 270 -80 1 0 {name=p21 sig_type=std_logic lab=vout_s1}
C {devices/code_shown.sym} -1300 -170 0 0 {name=s1 only_toplevel=false value="

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
print vout vs vout_s1
.endc

"}
C {capa.sym} 550 120 0 0 {name=C1
m=1
value=1e-12
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 670 130 0 0 {name=Ro_pt
value=1100000000
footprint=1206
device=resistor
m=1}
