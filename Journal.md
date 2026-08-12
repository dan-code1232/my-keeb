# This is my keep journal of the development and learning process(hopefully learning... we will see!)

## Section 1 (2hrs)
During this time I read through all the docs and planned my board. 

My initial designs are a 65 percent layout, with the pi on the top left and an oled screen on the top right:

![Screenshot 2026-08-04 123623](https://github.com/user-attachments/assets/9604cc8a-5d60-4ca1-8e74-e2075ccb4de9)

I then made a matrix schech to be sure of what i was going to design in kcad
matrix-sketch:

![Screenshot 2026-08-04 123416](https://github.com/user-attachments/assets/f6d8d2ec-7280-4650-963e-980e08ca5451)


## Section 2(1.5 hours)
I updated my sketch to include better use of space and also to add macropad-style multi function keys and volume buttons(im not going to include a rotary encoder):

<img width="1752" height="788" alt="Screenshot 2026-08-04 202955" src="https://github.com/user-attachments/assets/ab6a711d-363e-403d-9203-d4a226c20bd5" />

I then made a start on the process of transferring my design into the schematic editor with some mistakes that i found out later with numbering the rows and collums of the matrix:

<img width="860" height="350" alt="Screenshot 2026-08-04 203522" src="https://github.com/user-attachments/assets/1c3ecce1-7cc4-4da3-864b-b61085001f05" />



<img width="1767" height="537" alt="Screenshot 2026-08-04 203545" src="https://github.com/user-attachments/assets/d4c480d3-6902-4488-9d58-292097f35a89" />

And i wired the pi:  

<img width="580" height="688" alt="image" src="https://github.com/user-attachments/assets/036be638-f46f-4eed-9229-e70d7bdb0ffa" />

Then i found a problem with the wiring of each sweich and had to redo it to find the problems.

<img width="360" height="197" alt="image" src="https://github.com/user-attachments/assets/83908ee4-731f-4781-9d0d-299f92ee5e6a" />

 After fixing this i ran eec and got 27 errors, which consited mainly of uncorrected wires.This step was very tedious to fix all the errors systemaltically, but i completed it in the end:
 <img width="1036" height="1062" alt="Screenshot 2026-08-05 194456" src="https://github.com/user-attachments/assets/c9597d30-ecb7-4cc6-87f7-351c16a41394" />

## Section 3(1 hour)
After this I started aranging it on the pcb editor,howvever I got an error at first due to me opening the file in the wrong way:
<img width="1917" height="1078" alt="Screenshot 2026-08-12 084805" src="https://github.com/user-attachments/assets/13489afd-f9e6-4769-8b55-223147decac9" />
After fixing this i continued arranging the pcb
<img width="203" height="142" alt="Screenshot 2026-08-05 060318" src="https://github.com/user-attachments/assets/a2393825-3b2f-4336-8d30-13b347c00b19" />
