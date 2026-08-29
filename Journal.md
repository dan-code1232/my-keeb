# This is my keep journal of the development and learning process (hopefully learning... we will see!)
# TOTAL OF 23 hours.. i didnt make the full day....
## Section 1 (2hrs)
During this time I read through all the docs and planned my board. 

My initial designs are a 65 percent layout, with the Pi on the top left and an OLED screen on the top right:

![Screenshot 2026-08-04 123623](https://github.com/user-attachments/assets/9604cc8a-5d60-4ca1-8e74-e2075ccb4de9)

I then made a matrix sketch to be sure of what I was going to design in KiCad.
Matrix-sketch:

![Screenshot 2026-08-04 123416](https://github.com/user-attachments/assets/f6d8d2ec-7280-4650-963e-980e08ca5451)


## Section 2 (2 hours) - the funnest part!
I updated my sketch to include better use of space and also to add macropad-style multi-function keys and volume buttons (I'm not going to include a rotary encoder):
I  then did a lot of research about mechanical keyboards, pcb design and how to make different keyboards sound different.
<img width="1752" height="788" alt="Screenshot 2026-08-04 202955" src="https://github.com/user-attachments/assets/ab6a711d-363e-403d-9203-d4a226c20bd5" />

I then made a start on the process of transferring my design into the schematic editor with some mistakes that I found out later with numbering the rows and columns of the matrix:

<img width="860" height="350" alt="Screenshot 2026-08-04 203522" src="https://github.com/user-attachments/assets/1c3ecce1-7cc4-4da3-864b-b61085001f05" />

<img width="1767" height="537" alt="Screenshot 2026-08-04 203545" src="https://github.com/user-attachments/assets/d4c480d3-6902-4488-9d58-292097f35a89" />

And I wired the Pi:  

<img width="580" height="688" alt="image" src="https://github.com/user-attachments/assets/036be638-f46f-4eed-9229-e70d7bdb0ffa" />

Then I found a problem with the wiring of each switch and had to redo it to find the problems.

<img width="360" height="197" alt="image" src="https://github.com/user-attachments/assets/83908ee4-731f-4781-9d0d-299f92ee5e6a" />

After fixing this I ran ERC and got 27 errors, which consisted mainly of unconnected wires. This step was very tedious to fix all the errors systematically, but I completed it in the end:

<img width="1036" height="1062" alt="Screenshot 2026-08-05 194456" src="https://github.com/user-attachments/assets/c9597d30-ecb7-4cc6-87f7-351c16a41394" />

## Section 3 (4 hours) - a lot of mistakes
After this I started arranging it on the PCB editor, however I got an error at first due to me opening the file in the wrong way:

<img width="1917" height="1078" alt="Screenshot 2026-08-12 084805" src="https://github.com/user-attachments/assets/13489afd-f9e6-4769-8b55-223147decac9" />

After fixing this I continued arranging the PCB which took some time as there seemed to be too many keys at first... 

<img width="203" height="142" alt="Screenshot 2026-08-05 060318" src="https://github.com/user-attachments/assets/a2393825-3b2f-4336-8d30-13b347c00b19" />

This turned out to be because in the very first switch I made, I accidentally put 2 switches on top of each other so I went back to the schematic again to delete all of the duplicates.

<img width="899" height="528" alt="Screenshot 2026-08-21 091115" src="https://github.com/user-attachments/assets/8008af1c-0267-43a4-b065-099f35543a8b" />

Finally I got it in rows but still there were too many keys if I wanted to have keys that weren't 1u. However I ignored this completely and continued.. which was a mistake. 

## Section 4: Layout Math & Routing (9 hrs) - I took a loooooong time
I am still working on getting the layout perfectly aligned. Placing the switches on the 19.05mm grid takes a lot of fiddling. I kept messing up the stagger by trying to drag the whole row, which ruined the right edge alignment thing and the grid that I used to place the switches also messed me up a lot. I eventually realized the trick is to just use the wide keys on the left (like 1.5u Tab and 1.75u Caps) to push the letters over so it looks good. I used the Move Exactly tool to get the math perfect so both the left and right edges are completely straight, as the keycap set which I wanted already had the suggestion.

I figured out how to do the wide keys (like the 2u Backspace and 6.25u Spacebar) by just centering a 1u switch and leaving a gap for the stabilizers. I also caught a massive mistake, I had not planned

<img width="519" height="308" alt="Screenshot 2026-08-23 182820" src="https://github.com/user-attachments/assets/8e9c260f-5432-4f57-bff5-11ab8aa253c9" />

Which took a long time..

<img width="1062" height="593" alt="Screenshot 2026-08-25 121535" src="https://github.com/user-attachments/assets/52ca99fe-928e-4c0f-88cf-a87741294b57" />

and longer...

<img width="1077" height="612" alt="Screenshot 2026-08-25 121544" src="https://github.com/user-attachments/assets/26c60c1b-6f46-4c8d-8a9f-8f869cb10221" />

but I made it! and here is the finished work of art!

<img width="1225" height="790" alt="Screenshot 2026-08-25 130413" src="https://github.com/user-attachments/assets/501e32b7-759d-4918-8f61-797deea92ba7" />

until I ran DRC and got a terrible amount of errors but then I got it down to only 

<img width="502" height="433" alt="Screenshot 2026-08-25 132137" src="https://github.com/user-attachments/assets/39011105-f74e-4073-be9f-d2b764f984e8" />

these

<img width="882" height="897" alt="Screenshot 2026-08-04 170221" src="https://github.com/user-attachments/assets/1ad6f6c8-611c-4ef6-8c1e-10d95d89759d" />

Routing the copper traces was a nightmare. I routed the columns on the front layer and the rows on the back layer. It looks a bit messy, but as long as it passes the DRC I am happy. The ground thing was super satisfying. I put the empty space with copper on both layers and hit B to fill it. I still had some GND errors because two pads are trapped on little copper islands. I am going to fix them later... hopefully.

I decided to go with a sandwich mount for the case, so I did not bother putting mounting holes on the board. The screws will just pass through the 5mm border around the edge. I exported the board as a STEP file and I am now moving over to Fusion 360 to start designing the 3D case. 

## Section 5: Case Design (5hrs)
To do the case design I then opened up the online software Onshape. At first I played around a bit with the tools as I had never used the software before. After this I designed a simple 3D tray with a solid base which would act as my case. I then decided that if I printed a plate it might become stressed and crack over time so I changed my PCB to have mounting holes, and fixed some errors in the PCB at the same time. I then had to rebuild my case again with the new dimensions, which took a while, as the Onshape server kept lagging. Using the placement of the holes on the PCB I constructed several towers for the PCB to be able to screw into. After this I added fillets to all the corners to make it look better and then I used the assembly tab to make sure everything fit, which it did perfectly. Finally I used the split tool to split it in half. Overall I'm happy with my case design, but I think that it could be more detailed.

ZERO ERRORS!!!

<img width="552" height="660" alt="Screenshot 2026-08-28 140950" src="https://github.com/user-attachments/assets/41e6952e-6ebc-4a4f-a4d7-ead10c901f87" />

my design:

<img width="1568" height="800" alt="Screenshot 2026-08-28 091401" src="https://github.com/user-attachments/assets/5270567c-d7da-4644-a6a8-470c189f3428" />

<img width="448" height="372" alt="Screenshot 2026-08-28 123432" src="https://github.com/user-attachments/assets/4b3c100c-a853-4a8a-9b1f-3e60b4181c83" />

<img width="1902" height="927" alt="Screenshot 2026-08-27 105413" src="https://github.com/user-attachments/assets/3e11acfb-d6c4-45be-ad58-7a6a08c9767d" />

## Section 6: Silkscreens9(1 HOURS)
For my silkscreens I decided to try to make the board really good as the JLCPCB company has a minimum order of 5. One of my favorite shows is Star Wars so I added a rebel logo and an imperial one on the opposite sides. Also I put a simple logo on with my initials in it.
heres my logo:<img width="802" height="746" alt="Screenshot 2026-08-28 155328" src="https://github.com/user-attachments/assets/01b0adb8-def9-40b6-825c-210fb06c7cca" />

And the silkscreen
<img width="1155" height="665" alt="Screenshot 2026-08-28 160851" src="https://github.com/user-attachments/assets/8d4c0bd3-17e0-4b76-b6b9-d4bcf2b59831" />
after this i put it into jcpcb and played around with the settings until  i was happy:
<img width="1823" height="972" alt="Screenshot 2026-08-28 174512" src="https://github.com/user-attachments/assets/b1a3935e-afaa-4580-b565-bc42df9cb9de" />

and the 3d editor was cool
<img width="982" height="645" alt="Screenshot 2026-08-28 184101" src="https://github.com/user-attachments/assets/fb7ba4e8-4cb1-4c5d-9a9d-f589929c8f0f" />
