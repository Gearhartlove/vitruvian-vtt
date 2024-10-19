import { invoke } from "@tauri-apps/api/core"
import { useEffect, useState } from "react";


// TODO: create the typescript version of the objects in rust, could just us the TS libarry for this btw

// create a generic type script object
export abstract class CharacterBuilder {
    public static Home = () => {

        const navigateToAncestryAndHeritageSelection = () => {
            console.log("Navigating to ancestry and heritage selection")
            window.location.href = "/character-builder/ancestry-and-heritage"
        }

        return (
            <div className="py-10">
                <div className="flex flex-col gap-6 justify-center items-center px-56">
                    <h1>Creating a character</h1>
                    <button
                        onClick={navigateToAncestryAndHeritageSelection}
                        style={{ border: "2px solid white" }}
                    >
                        Select Ancestry and Heritage
                    </button>
                </div>
            </div>
        );
    }

    public static navigateToBackgroundSelection = () => {
        console.log("Navigating to background selection")
        window.location.href = "/character-builder/background"
    }

    public static AncestryAndHeritage = () => {
        const [ancestries, setAncestries] = useState<string>("");

        useEffect(() => {
            // Call the Rust function and update the state with the result
            invoke<string>('get_ancestries')
                .then((result) => {
                    setAncestries(result);
                })
                .catch((error) => {
                    console.error("Error invoking get_ancestries:", error);
                });
        }, []);

        return (
            <div className="py-10">
                <div className="flex flex-col gap-6 justify-center items-center px-56">
                    <h1>Ancestry and Heritage</h1>
                    <p
                        style={{ fontSize: "32px" }}
                    >Dwarf Ancestry Data Ingested From Rust</p>
                    <p>{ancestries}</p>
                    <br></br>
                    <button
                        onClick={CharacterBuilder.navigateToBackgroundSelection}
                        style={{ border: "2px solid white" }}
                    >Select Background</button>
                </div>
            </div>
        );
    }

    public static navigateToClassSelection = () => {
        console.log("Navigating to class selection")
        window.location.href = "/character-builder/class"
    }

    public static Background = () => {
        return (
            <div className="py-10">
                <div className="flex flex-col gap-6 justify-center items-center px-56">
                    <h1>Background</h1>
                    <button
                        onClick={CharacterBuilder.navigateToClassSelection}
                        style={{ border: "2px solid white" }}
                    >Select Class</button>
                </div>
            </div>
        );
    }

    public static Class = () => {
        return (
            <div className="py-10">
                <div className="flex flex-col gap-6 justify-center items-center px-56">
                    <h1>Class</h1>
                    <button
                        style={{ border: "2px solid white" }}
                    >Finish</button>
                </div>
            </div>
        )
    }
}
