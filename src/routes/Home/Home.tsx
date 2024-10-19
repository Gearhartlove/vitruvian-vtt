import { Entity } from "../../types/gen/Entity"
import CharacterPortrait from "../../components/ChracterPortrait"

const exampleEntities: Entity[] = [
  {
    Name: "John Doe",
    Damage: "D12"
  },
  {
    Name: "Dilf Laungrin",
    Damage: "D100"
  },
  {
    Name: "Jane Doe",
    Damage: "D6"
  },
  {
    Name: "Jane Doe",
    Damage: "D6"
  },
  {
    Name: "Jane Doe",
    Damage: "D6"
  },
  {
    Name: "Jane Doe",
    Damage: "D6"
  },
  {
    Name: "Jane Doe",
    Damage: "D6"
  },
  {
    Name: "Jane Doe",
    Damage: "D6"
  },
  {
    Name: "Jane Doe",
    Damage: "D6"
  }
]

const navigateToCharacterBuilder = () => {
  console.log("Navigating to character builder")
  window.location.href = "/character-builder/start"
}

const Home = () => {
  return (
    <div className="py-10">
      <div className="flex flex-col gap-6 justify-center items-center px-56">
        <button
          onClick={navigateToCharacterBuilder}
          style={{ border: "2px solid white" }}>
            Create a character</button>
      </div>
    </div>
  )
}

export default Home