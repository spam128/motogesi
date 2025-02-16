# Connect ESP8266-01 for UART communication


### Connecting ESP8266-01 to STM32 Blue Pill

| **ESP8266-01** | **STM32 Blue Pill** | **Opis** |
| --- | --- | --- |
| **3V3** | 3.3V | Zasilanie (nie podłączaj 5V!) |
| **GND** | GND | Masa |
| **TX** | PA3 (RX2) | ESP8266 wysyła dane do STM32 |
| **RX** | PA2 (TX2) | STM32 wysyła dane do ESP8266 |
| **EN** | 3.3V | **Wymagane!** Włącza ESP8266 |
| **RST** | (nie podłączać) | Reset (opcjonalnie do GND na chwilę, aby zresetować) |

Dodatkowo:
- **IO0** i **IO2** – zostaw niepodłączone, jeśli chcesz normalnie uruchomić ESP8266.
- Jeśli chcesz wprowadzić ESP8266 w **tryb flashowania**, podłącz **IO0 do GND** i zrestartuj moduł.


podaj teraz ta czesc biorac pod uwage ze:
ESP8266-01 | STM32 bluepill
TX | PA3(RX2)
RX | PA2 (TX2)

# Info
Diod state
- blinks
- turned off
- turned on - wifi panic