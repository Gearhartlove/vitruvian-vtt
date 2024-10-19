import {
  createBrowserRouter,
} from "react-router-dom";
import Home from "./Home/Home";
import { CharacterBuilder } from "./character_builder/character_builder";

const routes = createBrowserRouter([
  {
    path: "/",
    Component: Home,
  },
  {
    path: "/character-builder/start",
    Component: CharacterBuilder.Home
  },
  {
    path: "/character-builder/ancestry-and-heritage",
    Component: CharacterBuilder.AncestryAndHeritage
  },
  {
    path: "/character-builder/background",
    Component: CharacterBuilder.Background
  },
  {
    path: "/character-builder/class",
    Component: CharacterBuilder.Class
  }
])

export default routes;