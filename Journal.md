# This is my keep journal of the development and learning process(hopefully learning... we will see!)

## Section 1 (2hrs)
During this time I read through all the docs and planned my board. 

My initial designs are a 65 percent layout, with the pi on the top left and an oled screen on the top right:

![Screenshot 2026-08-04 123623](https://github.com/user-attachments/assets/9604cc8a-5d60-4ca1-8e74-e2075ccb4de9)

I then made a matrix schech to be sure of what i was going to design in kcad
matrix-sketch:

![Screenshot 2026-08-04 123416](https://github.com/user-attachments/assets/f6d8d2ec-7280-4650-963e-980e08ca5451)


## Section 2(1.5 hours)the funnest part!
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

## Section 3:(4 hour) a lot of mistakes
After this I started aranging it on the pcb editor,howvever I got an error at first due to me opening the file in the wrong way:

<img width="1917" height="1078" alt="Screenshot 2026-08-12 084805" src="https://github.com/user-attachments/assets/13489afd-f9e6-4769-8b55-223147decac9" />

After fixing this i continued arranging the pcb which took some time as there seemed to be too many keys at first... 

<img width="203" height="142" alt="Screenshot 2026-08-05 060318" src="https://github.com/user-attachments/assets/a2393825-3b2f-4336-8d30-13b347c00b19" />

This turne out to be because in the very first switch i made i accidentally put 2 switches ontop of each other so i went back to the schematic again to delete all of the duplicates.

<img width="899" height="528" alt="Screenshot 2026-08-21 091115" src="https://github.com/user-attachments/assets/8008af1c-0267-43a4-b065-099f35543a8b" />

Finally i got it in rows but still there were too many keys if i wanted to have keys that werent 1 u. howvere i ignored this completely and continued.. which was a mistake. 
## Section 4: Layout Math & Routing (9 hrs) i took a loooooong time
I am still working on getting the layout perfectly aligned. Placing the switches on the 19.05mm grid takes a lot of fiddling. I kept messing up the stagger by trying to drag the whole row, which ruined the right edge alignment thing and the grid that i used to place the switches also messed me up a lot. I eventually realized the trick is to just use the wide keys on the left (like 1.5u Tab and 1.75u Caps) to push the letters overso it looks good I used the move xactly tool to get the math perfect so both the left and right edges are completely straight, as the keycap set which i wanted already had the ssugestion 



I figured out how to do the wide keys (like the 2u Backspace and 6.25u Spacebar) by just centering a 1u switch and leaving a gap for the stabilizers. I also caught a massive mistake, i had not planned
[Insert screenshot of stabilizers placed here]
<img width="519" height="308" alt="Screenshot 2026-08-23 182820" src="https://github.com/user-attachments/assets/8e9c260f-5432-4f57-bff5-11ab8aa253c9" />

Which took a long time..

<img width="1062" height="593" alt="Screenshot 2026-08-25 121535" src="https://github.com/user-attachments/assets/52ca99fe-928e-4c0f-88cf-a87741294b57" />

and longer...

<img width="1077" height="612" alt="Screenshot 2026-08-25 121544" src="https://github.com/user-attachments/assets/26c60c1b-6f46-4c8d-8a9f-8f869cb10221" />

but i made it! and here is th finished work of art!

<img width="1225" height="790" alt="Screenshot 2026-08-25 130413" src="https://github.com/user-attachments/assets/501e32b7-759d-4918-8f61-797deea92ba7" />

until i ran drc and got a terrible amunt of errors but then i got it down to only 

<img width="502" height="433" alt="Screenshot 2026-08-25 132137" src="https://github.com/user-attachments/assets/39011105-f74e-4073-be9f-d2b764f984e8" />

these

<img width="882" height="897" alt="Screenshot 2026-08-04 170221" src="https://github.com/user-attachments/assets/1ad6f6c8-611c-4ef6-8c1e-10d95d89759d" />



Routing the copper traces was a nightmare. I routed the columns on the front layer and the rows on the back layer. It looks a bit messy, but as long as it passes the DRC I am happy the ground thing was super satisfying.i put the empty space with copper on both layers and hit B to fill it. I still had some GND errors because two pads are trapped on little copper islands. I am going to fix them later.....



I decided to go with a sandwich mount for the case, so I did not bother putting mounting holes on the board. The screws will just pass through the 5mm border around the edge. I exported the board as a STEP file and I am now moving over to Fusion 360 to start designing the 3D case. 

Taking a quick break right now to find some PCB mount stabilizers to add to my BOM.

