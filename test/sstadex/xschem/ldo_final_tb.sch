v {xschem version=3.4.5 file_version=1.2
}
G {}
K {}
V {}
S {}
E {}
N -10 -150 -10 -90 {
lab=vs}
N 170 -90 350 -90 {
lab=vs}
N 170 -90 170 -40 {
lab=vs}
N 310 -430 350 -430 {
lab=vdd}
N -10 -300 -10 -250 {
lab=#net1}
N 350 -270 550 -270 {
lab=vout_s1}
N 410 -140 410 -120 {
lab=vfb}
N 410 -210 410 -200 {
lab=vfb}
N 170 20 170 50 {
lab=vss}
N -10 -90 170 -90 {
lab=vs}
N -10 -430 90 -430 {
lab=vdd}
N 350 -300 350 -270 {
lab=vout_s1}
N 90 -430 310 -430 {
lab=vdd}
N 410 -200 410 -140 {
lab=vfb}
N 350 -330 350 -300 {
lab=vout_s1}
N -10 -340 -10 -300 {
lab=#net1}
N -10 -430 -10 -400 {
lab=vdd}
N 350 -340 350 -330 {
lab=vout_s1}
N 350 -430 350 -400 {
lab=vdd}
N 350 -270 350 -240 {
lab=vout_s1}
N 350 -180 350 -90 {
lab=vs}
N -10 -250 -10 -240 {
lab=#net1}
N -10 -180 -10 -150 {
lab=vs}
N 390 -210 410 -210 {
lab=vfb}
N -10 -210 350 -210 {
lab=vs}
N 190 -210 190 -90 {
lab=vs}
N 170 50 170 110 {
lab=vss}
N 30 -370 310 -370 {
lab=#net1}
N 170 -370 170 -310 {
lab=#net1}
N -10 -310 170 -310 {
lab=#net1}
N -60 -370 -10 -370 {
lab=vdd}
N -60 -430 -60 -370 {
lab=vdd}
N -60 -430 -10 -430 {
lab=vdd}
N 350 -370 400 -370 {
lab=vdd}
N 400 -430 400 -370 {
lab=vdd}
N 350 -430 400 -430 {
lab=vdd}
N 590 -430 590 -300 {
lab=vdd}
N 400 -430 590 -430 {
lab=vdd}
N 590 -270 620 -270 {
lab=vdd}
N 620 -320 620 -270 {
lab=vdd}
N 590 -320 620 -320 {
lab=vdd}
N 590 -210 640 -210 {
lab=vmid}
N 590 -240 590 -210 {
lab=vmid}
N 590 -210 590 -140 {
lab=vmid}
N 590 -70 590 110 {
lab=vss}
N 170 110 590 110 {
lab=vss}
N 590 -80 590 -70 {
lab=vss}
N 640 -210 770 -210 {
lab=vmid}
N 510 -210 590 -210 {
lab=vmid}
N 500 -210 510 -210 {
lab=vmid}
N 580 -430 810 -430 {
lab=vdd}
N 810 -180 810 -130 {
lab=vout}
N 810 -70 810 -40 {
lab=vfb}
N 810 20 810 40 {
lab=vss}
N 770 -60 810 -60 {
lab=vfb}
N 1030 -150 1030 -80 {
lab=vout}
N 930 -150 930 -80 {
lab=vout}
N 930 -20 930 30 {
lab=vss}
N 810 30 930 30 {
lab=vss}
N 930 30 1030 30 {
lab=vss}
N 1030 -20 1030 30 {
lab=vss}
N 810 -150 1110 -150 {
lab=vout}
N 810 -210 850 -210 {
lab=vdd}
N 850 -260 850 -210 {
lab=vdd}
N 810 -260 850 -260 {
lab=vdd}
N 810 -300 810 -240 {
lab=vdd}
N 810 -430 810 -360 {
lab=vdd}
N 810 -360 810 -300 {
lab=vdd}
C {devices/isource.sym} 170 -10 0 0 {name=I2 value=40e-6}
C {devices/lab_pin.sym} 170 -90 1 0 {name=p1 sig_type=std_logic lab=vs}
C {devices/lab_pin.sym} 1110 -150 2 0 {name=p4 sig_type=std_logic lab=vout}
C {devices/lab_pin.sym} 150 -430 1 0 {name=p6 sig_type=std_logic lab=vdd}
C {devices/lab_pin.sym} -50 -210 0 0 {name=p7 sig_type=std_logic lab=vn}
C {devices/lab_pin.sym} 170 110 3 0 {name=p9 sig_type=std_logic lab=vss}
C {sky130_fd_pr/nfet_01v8_lvt.sym} 370 -210 0 1 {name=M1
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
C {sky130_fd_pr/nfet_01v8_lvt.sym} -30 -210 0 0 {name=M2
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
C {sky130_fd_pr/pfet_01v8_lvt.sym} 330 -370 0 0 {name=M3
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
C {sky130_fd_pr/pfet_01v8_lvt.sym} 10 -370 0 1 {name=M4
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
C {sky130_fd_pr/pfet_01v8_lvt.sym} 570 -270 0 0 {name=M5
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
C {devices/isource.sym} 590 -110 0 0 {name=I1 value=20e-6}
C {devices/code_shown.sym} -890 -450 0 0 {name=s1 only_toplevel=false value="

.lib /opt/pdks/sky130A/libs.tech/ngspice/sky130.lib.spice tt

Vref vn 0 dc 0.9
Vdd vdd 0 1.8 ac 1
Vss vss 0 0

.control
ac dec 10 1 100G
plot vdb(vout) 
plot phase(vout)
op
print vout vmid vout_s1 vs
print @m.xm6.msky130_fd_pr__pfet_01v8_lvt[id]
.endc

.control
tran 10n 1u
plot vout vs vmid Vmeas
.endc

"}
C {capa.sym} 500 -240 0 0 {name=C8
m=1
value=3e-12
footprint=1206
device="ceramic capacitor"}
C {sky130_fd_pr/pfet_01v8_lvt.sym} 790 -210 0 0 {name=M6
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
C {devices/res.sym} 810 -100 0 0 {name=R1
value=100000
footprint=1206
device=resistor
m=1}
C {devices/res.sym} 810 -10 0 0 {name=R2
value=300000
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 770 -60 0 0 {name=p5 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 810 40 3 0 {name=p2 sig_type=std_logic lab=vss}
C {capa.sym} 930 -50 0 0 {name=Cl
m=1
value=50e-12
footprint=1206
device="ceramic capacitor"}
C {devices/res.sym} 1030 -50 0 0 {name=R3
value=12
footprint=1206
device=resistor
m=1}
C {devices/lab_pin.sym} 410 -120 3 0 {name=p3 sig_type=std_logic lab=vfb}
C {devices/lab_pin.sym} 710 -210 3 0 {name=p8 sig_type=std_logic lab=vmid}
C {devices/lab_pin.sym} 440 -270 1 0 {name=p10 sig_type=std_logic lab=vout_s1}
