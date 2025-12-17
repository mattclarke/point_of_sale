### Features

- Sell an item reduces inventory
- Sell multiple items
- Apply area tax
- Tax-exempt items
- Add fixture for tests
- LOD violation?   assert_eq!(pos.display.get_text(), "$7.95");

Sell multiple items
- Sell 1 item on happy path - DONE
- Sell 3 items on happy path - DONE
- Barcode not found, enter price manually - WORK IN PROGRESS
    - Too many decimal points
    - Too many full stops
- Show total items and total price when finished - DONE 
- What happens if finished at 0 items - DONE
- What happens if we scan a million items - WON'T DO