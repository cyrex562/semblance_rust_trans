@echo off

:doscreen

cls


echo              浜様様様様様様様様様様様様様様様様様様様様様様様様様�
echo              �                  ImagiNation CD                   �
echo              把陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳陳�
echo              �                                                   �
echo              �    This CD contains the software for The          �
echo              � ImagiNation Network.                              �
echo              �                                                   �
echo              �    To install ImagiNation onto your hard disk,    �
echo              � press 1.                                          �
echo              �                                                   �
echo              �    To exit to DOS press 2.                        �
echo              �                                                   �
echo              �                                                   �
echo              �               1 - Install ImagiNation             �
echo              �               2 - Exit to DOS                     �
echo              �                                                   �
echo              �                                                   �
echo              藩様様様様様様様様様様様様様様様様様様様様様様様様様�


..\what CE 12

if "%what%" == "1" goto install_INN

if "%what%" == "2" goto drop2dos
if "%what%" == "" goto drop2dos

goto doscreen

:install_INN
install

:drop2dos
cls
