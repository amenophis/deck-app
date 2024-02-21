import {useQuery} from "react-query";
import {getDecks, setButtonImage} from "./api/api.ts";
import {useState} from "react";

function App() {
  const [currentDeck, setCurrentDeck] = useState<string>("")

  const { data: decks, error, isLoading } = useQuery('get_decks', getDecks);

  if (isLoading) return <div>Fetching decks...</div>;
  if (error) return <div>An error occurred: {error.message}</div>;

  function SetImage()
  {
    setButtonImage(currentDeck, 1);
  }

  return (
    <div>
      <h1>Amenophis Deck App</h1>

      Device list:
      {
        <select onChange={e => setCurrentDeck(e.target.value)}>
          <option value="">Please select a deck</option>
          {
            decks && decks.map(deck => (
              <option key={deck.serial} value={deck.serial} >
                {`${deck.serial}`}
              </option>
            ))
          }
        </select>
      }

      { currentDeck }

      <button onClick={SetImage}>Set image</button>
    </div>
  )
    ;
}

export default App;
