---
title: "Keeb keyboard"
github: "https://github.com/dan-code1232/my-keeb"
description: "This YSWS was my first hack club project. I learnt about hackclub from online and I had a great time building and completing this project. During this project I made a custom keyboard with programmable macro keys and a case!"
created_at: "2026-10-17"
total_time: "37h"
---

# August 4, 2026: Section 1

During this time I read through all the docs and planned my board. 

My initial designs are a 65 percent layout, with the Pi on the top left and an OLED screen on the top right:

![Screenshot 2026-08-04 123623](https://fabricate.hackclub-assets.com/c04e25c1234f44638a6f5baccf61ef614f91095683b1c5d8e98a0777048d5c92/image-png/9604cc8a-5d60-4ca1-8e74-e2075ccb4de9.png)

I then made a matrix sketch to be sure of what I was going to design in KiCad.
Matrix-sketch:

![Screenshot 2026-08-04 123416](https://fabricate.hackclub-assets.com/1e802046d590f6a1e3c06e37216c123807ec5eb67957c9d4892ff4b8c249a34c/image-png/f6d8d2ec-7280-4650-963e-980e08ca5451.png)

**Total time spent: 5h**

# August 4, 2026: Section 2 - the funnest part!

I updated my sketch to include better use of space and also to add macropad-style multi-function keys and volume buttons (I'm not going to include a rotary encoder). I then did a lot of research about mechanical keyboards, PCB design and how to make different keyboards sound different.

![Screenshot 2026-08-04 202955](https://fabricate.hackclub-assets.com/f9b222e61f5bdf14c4a1e8314847394f14ef170ee0d274f0e958d034c2d087f4/image-png/ab6a711d-363e-403d-9203-d4a226c20bd5.png)

I then made a start on the process of transferring my design into the schematic editor with some mistakes that I found out later with numbering the rows and columns of the matrix:

![Screenshot 2026-08-04 203522](https://fabricate.hackclub-assets.com/837e25994ebd4a4f27bdefa5fc6a00c55f07617e83059ad1eb9308f54c958e95/image-png/1c3ecce1-7cc4-4da3-864b-b61085001f05.png)

![Screenshot 2026-08-04 203545](https://fabricate.hackclub-assets.com/9eddea7a08d8e6d5a58d1dd5c1752e64d53c3c5481a2d4bd6bf0c7954259295c/image-png/d4c480d3-6902-4488-9d58-292097f35a89.png)

And I wired the Pi:  

![image](https://fabricate.hackclub-assets.com/3f4f936ee997acf9063762604819a880f8363d2c6a8c8e0affce9f31348d2e68/image-png/036be638-f46f-4eed-9229-e70d7bdb0ffa.png)

Then I found a problem with the wiring of each switch and had to redo it to find the problems.

![image](https://fabricate.hackclub-assets.com/198cb3c4badc5714ae0e6654a4249f49931b78d42b4c4d6fd2d0c9d27f2c129f/image-png/83908ee4-731f-4781-9d0d-299f92ee5e6a.png)

After fixing this I ran ERC and got 27 errors, which consisted mainly of unconnected wires. This step was very tedious to fix all the errors systematically, but I completed it in the end:

![Screenshot 2026-08-05 194456](https://fabricate.hackclub-assets.com/7a1397bffa6b517f43463d77daf6fb6708ae87b513b408ae729cd8ebc62c39b9/image-png/c9597d30-ecb7-4cc6-87f7-351c16a41394.png)

**Total time spent: 2h 30m**

# August 12, 2026: Section 3

After this I started arranging it on the PCB editor, however I got an error at first due to me opening the file in the wrong way:

![Screenshot 2026-08-12 084805](https://fabricate.hackclub-assets.com/f233eb1c6c7188614bb02febe09eabe251a87d6b4cefb81a6c0dda8a7d1d41b7/image-png/13489afd-f9e6-4769-8b55-223147decac9.png)

After fixing this I continued arranging the PCB which took some time as there seemed to be too many keys at first... 

![Screenshot 2026-08-05 060318](https://fabricate.hackclub-assets.com/e0afdcf9c2da0f3cfac4f418d5295f26a8062fff14b777bdddc44177e8489189/image-png/a2393825-3b2f-4336-8d30-13b347c00b19.png)

This turned out to be because in the very first switch I made, I accidentally put 2 switches on top of each other so I went back to the schematic again to delete all of the duplicates.

![Screenshot 2026-08-21 091115](https://fabricate.hackclub-assets.com/07af2cd30fdcd7032aaa18303c9e575038faad6079937579e7aa7dfafc9c2d71/image-png/8008af1c-0267-43a4-b065-099f35543a8b.png)

Finally I got it in rows but still there were too many keys if I wanted to have keys that weren't 1u. However I ignored this completely and continued.. which was a mistake.

**Total time spent: 5h**

# August 23, 2026: Section 4: Layout Math & Routing

I am still working on getting the layout perfectly aligned. Placing the switches on the 19.05mm grid takes a lot of fiddling. I kept messing up the stagger by trying to drag the whole row, which ruined the right edge alignment thing and the grid that I used to place the switches also messed me up a lot. I eventually realized the trick is to just use the wide keys on the left (like 1.5u Tab and 1.75u Caps) to push the letters over so it looks good. I used the Move Exactly tool to get the math perfect so both the left and right edges are completely straight, as the keycap set which I wanted already had the suggestion.

I figured out how to do the wide keys (like the 2u Backspace and 6.25u Spacebar) by just centering a 1u switch and leaving a gap for the stabilizers. I also caught a massive mistake, I had not planned

![Screenshot 2026-08-23 182820](https://fabricate.hackclub-assets.com/dc52479b2833411d041867472869094f4c868b4c29126373b9d09305a196d56d/image-png/8e9c260f-5432-4f57-bff5-11ab8aa253c9.png)

Which took a long time..

![Screenshot 2026-08-25 121535](https://fabricate.hackclub-assets.com/9a51d7248375cd61b4ad1593b8ac6cd023fc19f3c658fdbe313ba98225a7f62f/image-png/52ca99fe-928e-4c0f-88cf-a87741294b57.png)

and longer...

![Screenshot 2026-08-25 121544](https://fabricate.hackclub-assets.com/167f58bd00672d46cc0e85d1449fb0880f543a9b141da21a47a69e6b4a7e06ea/image-png/26c60c1b-6f46-4c8d-8a9f-8f869cb10221.png)

but I made it! and here is the finished work of art!

![Screenshot 2026-08-25 130413](https://fabricate.hackclub-assets.com/b838d1516cc715634e61513adcf61266af64758972c0706739da52670e4e38c8/image-png/501e32b7-759d-4918-8f61-797deea92ba7.png)

until I ran DRC and got a terrible amount of errors but then I got it down to only 

![Screenshot 2026-08-25 132137](https://fabricate.hackclub-assets.com/44f557174446049b1980b5851f6f41318b729fb4af397f7a72f1a510306603e7/image-png/39011105-f74e-4073-be9f-d2b764f984e8.png)

these

![Screenshot 2026-08-04 170221](https://fabricate.hackclub-assets.com/45ece31e63f2d3283b805666ab791919243a83b80c03bba48aff4b69a348b81f/image-png/1ad6f6c8-611c-4ef6-8c1e-10d95d89759d.png)

Routing the copper traces was a nightmare. I routed the columns on the front layer and the rows on the back layer. It looks a bit messy, but as long as it passes the DRC I am happy. The ground thing was super satisfying. I put the empty space with copper on both layers and hit B to fill it. I still had some GND errors because two pads are trapped on little copper islands. I am going to fix them later... hopefully.

I decided to go with a sandwich mount for the case, so I did not bother putting mounting holes on the board. The screws will just pass through the 5mm border around the edge. I exported the board as a STEP file and I am now moving over to Fusion 360 to start designing the 3D case.

**Total time spent: 9h 30m**

# August 28, 2026: Section 5: Case Design (6hrs)

To do the case design I then opened up the online software Onshape. At first I played around a bit with the tools as I had never used the software before. After this I designed a simple 3D tray with a solid base which would act as my case. I then decided that if I printed a plate it might become stressed and crack over time so I changed my PCB to have mounting holes, and fixed some errors in the PCB at the same time. I then had to rebuild my case again with the new dimensions, which took a while, as the Onshape server kept lagging. Using the placement of the holes on the PCB I constructed several towers for the PCB to be able to screw into. After this I added fillets to all the corners to make it look better and then I used the assembly tab to make sure everything fit, which it did perfectly. Finally I used the split tool to split it in half. Overall I'm happy with my case design, but I think that it could be more detailed.

ZERO ERRORS!!!::::

![Screenshot 2026-08-28 140950](https://fabricate.hackclub-assets.com/19242e61b9ff2973f2d167bb1d393c788c0580731de2f4cadd8025a4ad60b279/image-png/41e6952e-6ebc-4a4f-a4d7-ead10c901f87.png)

my design:

![Screenshot 2026-08-28 091401](https://fabricate.hackclub-assets.com/7e036179b4dda70c5c7f1083bf3dde96b79828711633b3363add4585ed5a5bf3/image-png/5270567c-d7da-4644-a6a8-470c189f3428.png)

![Screenshot 2026-08-28 123432](https://fabricate.hackclub-assets.com/b604a2e5b9bedff1f07fb7b34748c88a6afc54084c3d66299599ab911a4caed7/image-png/4b3c100c-a853-4a8a-9b1f-3e60b4181c83.png)

![Screenshot 2026-08-27 105413](https://fabricate.hackclub-assets.com/4b3bc27747b69f8f4d11b8934bf74fd70336a2b892b53a58955d51b672a50e78/image-png/3e11acfb-d6c4-45be-ad58-7a6a08c9767d.png)

**Total time spent: 6h**

# August 28, 2026: Section 6: Silkscreens +firmware

For my silkscreens I decided to try to make the board really good as the JLCPCB company has a minimum order of 5. One of my favorite shows is Star Wars so I added a rebel logo and an imperial one on the opposite sides. Also I put a simple logo on with my initials in it.
Here's my logo:
![Screenshot 2026-08-28 155328](https://fabricate.hackclub-assets.com/8c6466f46db5df511b916d18bb3e1c4ccb05c4491a50dd479911063c8e76e081/image-png/01b0adb8-def9-40b6-825c-210fb06c7cca.png)

And the silkscreen:
![Screenshot 2026-08-28 160851](https://fabricate.hackclub-assets.com/77d698fc1b58fa93f5759fab3010f00e2d50081f34cb1db0c7995f7f82da0168/image-png/8d4c0bd3-17e0-4b76-b6b9-d4bcf2b59831.png)

After this I put it into JLCPCB and played around with the settings until I was happy:
![Screenshot 2026-08-28 174512](https://fabricate.hackclub-assets.com/e11a513055b8103cc70e0a35b626de23059b533f4cc7dc43636e20448d5e3927/image-png/b1a3935e-afaa-4580-b565-bc42df9cb9de.png)

And the 3D editor was cool:
![Screenshot 2026-08-28 184101](https://fabricate.hackclub-assets.com/a8a691cea705f90bacc2b830334a211521888e96d61c3102f28ce562f415d3b5/image-png/fb7ba4e8-4cb1-4c5d-9a9d-f589929c8f0f.png)

I was scared at first by writing all the firmware but i found a template in RMK and copied it whilest changing everything like the rows and collms and keymap etc,which took less time then i was expecting.

**Total time spent: 4h 30m**

# August 31, 2026: Section 8: BOM & Submission(4.5 hours approx)

finished compiling my  bom and getting all my files exported and zipped and everything.I spent a long time choosing parts and compatibilitu checking between them and making sure i had the cheapest yet best deals and good reliable parts. Im finally done!
![Screenshot 2026-08-31 073237](https://fabricate.hackclub-assets.com/739e305ae690964b3255dd0cd1f7569b3390cd5495e9b2181439c5a99ce779fe/image-png/743c9b87-08b8-400c-adfc-083861562101.png)
![Screenshot 2026-08-24 091937](https://fabricate.hackclub-assets.com/22a0c8490893d4cf3dd634280ae0634c85410dd3b541f014c3acf338a04eaf44/image-png/ab69f9df-6ac4-4334-9bfa-18d9f42100db.png)
![Screenshot 2026-08-24 101943](https://fabricate.hackclub-assets.com/200c91eec9ecc30963f6889dc25d2fedc0be49fb97fe566143dc37254ec1debd/image-png/51d60de9-d64c-41c3-8ce4-9659095d2e18.png)
![Screenshot 2026-08-31 073302](https://fabricate.hackclub-assets.com/140722c50267701919162e7fe5eff6c372a8eb4ec9fd856a6e1877af15f75696/image-png/e0d616b2-5f70-4e87-803c-bf70b292a573.png)

THE END

**Total time spent: 4h 30m**
