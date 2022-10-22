#Backend for the wordle clone from my personal website.
When the user clicks on the button "Play Game" the browser requests a word from the /wordle/get_word endpoint, 
the response that it receives contains the target word as its body.
#Then, the button label will change to "Check Word".
#Everytime the user submits a word, the app sends a request to /wordle/check_word/{word}.
#If the word is a valid word, the server sends a response with 'true' as body.
#If the word is not valid, the server sends a response with 'false' as body.
