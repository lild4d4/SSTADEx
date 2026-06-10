v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -350 -50 -350 10 {
lab=vs}
N -170 10 10 10 {
lab=vs}
N -170 10 -170 60 {
lab=vs}
N -30 -330 10 -330 {
lab=vdd}
N -350 -200 -350 -150 {
lab=#net1}
N 10 -170 210 -170 {
lab=vout_s1}
N -480 -110 -480 -80 {
lab=vn}
N -480 -20 -480 0 {
lab=vn}
N -480 -110 -390 -110 {
lab=vn}
N 70 -40 70 -20 {
lab=vfb}
N 70 -110 70 -100 {
lab=vfb}
N -170 120 -170 150 {
lab=vss}
N -350 10 -170 10 {
lab=vs}
N -350 -330 -250 -330 {
lab=vdd}
N 10 -200 10 -170 {
lab=vout_s1}
N -250 -330 -30 -330 {
lab=vdd}
N -480 -80 -480 -20 {
lab=vn}
N 70 -100 70 -40 {
lab=vfb}
N 10 -230 10 -200 {
lab=vout_s1}
N -350 -240 -350 -200 {
lab=#net1}
N -350 -330 -350 -300 {
lab=vdd}
N 10 -240 10 -230 {
lab=vout_s1}
N 10 -330 10 -300 {
lab=vdd}
N 10 -170 10 -140 {
lab=vout_s1}
N 10 -80 10 10 {
lab=vs}
N -350 -150 -350 -140 {
lab=#net1}
N -350 -80 -350 -50 {
lab=vs}
N 50 -110 70 -110 {
lab=vfb}
N -350 -110 10 -110 {
lab=vs}
N -150 -110 -150 10 {
lab=vs}
N -170 150 -170 210 {
lab=vss}
N -310 -270 -30 -270 {
lab=#net1}
N -170 -270 -170 -210 {
lab=#net1}
N -350 -210 -170 -210 {
lab=#net1}
N -400 -270 -350 -270 {
lab=vdd}
N -400 -330 -400 -270 {
lab=vdd}
N -400 -330 -350 -330 {
lab=vdd}
N 10 -270 60 -270 {
lab=vdd}
N 60 -330 60 -270 {
lab=vdd}
N 10 -330 60 -330 {
lab=vdd}
N 250 -330 250 -200 {
lab=vdd}
N 60 -330 250 -330 {
lab=vdd}
N 250 -170 280 -170 {
lab=vdd}
N 280 -220 280 -170 {
lab=vdd}
N 250 -220 280 -220 {
lab=vdd}
N 250 -110 300 -110 {
lab=vmid}
N 250 -140 250 -110 {
lab=vmid}
N 250 -110 250 -40 {
lab=vmid}
N 250 30 250 210 {
lab=vss}
N -170 210 250 210 {
lab=vss}
N 250 20 250 30 {
lab=vss}
N 300 -110 430 -110 {
lab=vmid}
N 170 -110 250 -110 {
lab=vmid}
N 160 -110 170 -110 {
lab=vmid}
N 240 -330 470 -330 {
lab=vdd}
N 470 -80 470 -30 {
lab=vout}
N 470 30 470 60 {
lab=vfb}
N 470 120 470 140 {
lab=vss}
N 430 40 470 40 {
lab=vfb}
N 690 -50 690 20 {
lab=vout}
N 590 -50 590 20 {
lab=vout}
N 590 80 590 130 {
lab=vss}
N 470 130 590 130 {
lab=vss}
N 590 130 690 130 {
lab=vss}
N 690 80 690 130 {
lab=vss}
N 470 -50 770 -50 {
lab=vout}
N 470 -110 510 -110 {
lab=vdd}
N 510 -160 510 -110 {
lab=vdd}
N 470 -160 510 -160 {
lab=vdd}
N 470 -200 470 -140 {
lab=vdd}
N 470 -330 470 -260 {
lab=vdd}
N 470 -260 470 -200 {
lab=vdd}
C {devices/isource.sym} -170 90 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} -170 10 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 770 -50 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} -190 -330 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -480 0 3 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} -170 210 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sky130_fd_pr/nfet_01v8_lvt.sym} 30 -110 0 1 {name=M1
W=5.51
L=0.4
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
C {sky130_fd_pr/nfet_01v8_lvt.sym} -370 -110 0 0 {name=M2
W=5.51
L=0.4
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
C {sky130_fd_pr/pfet_01v8_lvt.sym} -10 -270 0 0 {name=M3
W=4.63
L=0.8
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
C {sky130_fd_pr/pfet_01v8_lvt.sym} -330 -270 0 1 {name=M4
W=4.63
L=0.8
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
C {sky130_fd_pr/pfet_01v8_lvt.sym} 230 -170 0 0 {name=M5
W=10.85
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
C {devices/isource.sym} 250 -10 0 0 {name=I1 value=20e-6}
C {capa.sym} 160 -140 0 0 {name=C8
m=1
value=3e-12
footprint=1206
device="ceramic capacitor"}
C {sky130_fd_pr/pfet_01v8_lvt.sym} 450 -110 0 0 {name=M6
W=12.565
L=0.4
nf=1
mult=1000
ad="'int((nf+1)/2) * W/nf * 0.29'" 
pd="'2*int((nf+1)/2) * (W/nf + 0.29)'"
as="'int((nf+2)/2) * W/nf * 0.29'" 
ps="'2*int((nf+2)/2) * (W/nf + 0.29)'"
nrd="'0.29 / W'" nrs="'0.29 / W'"
sa=0 sb=0 sd=0
model=pfet_01v8_lvt
spiceprefix=X
}
C {devices/res.sym} 470 0 0 0 {name=R1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 470 90 0 0 {name=R2
value=300000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 430 40 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 470 140 3 0 {name=p2 sig_type=std_logic lab=vss}
C {capa.sym} 590 50 0 0 {name=Cl
m=1
value=50e-12
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 690 50 0 0 {name=R3
value=12
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 70 -20 3 0 {name=p3 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 370 -110 3 0 {name=p8 sig_type=std_logic lab=vmid}
C {devices/lab_pin.sym} 100 -170 1 0 {name=p10 sig_type=std_logic lab=vout_s1}
C {devices/code_shown.sym} -1170 -280 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vref vn 0 0.9
Vdd vdd 0 1.8 ac 1
Vss vss 0 0

.control
ac dec 10 1 100G
plot vdb(vout) 
plot phase(vout)
op
print vout vmid vout_s1 vs
.endc

"}
