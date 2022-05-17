module Main exposing (..)

import Browser
import Html exposing (Html, h1, text)


type alias Model =
    { name : String }


view : Model -> Html ()
view model =
    h1 [] [ text ("Hello " ++ model.name ++ " World!!!") ]


main : Program () Model ()
main =
    Browser.sandbox
        { init = { name = "Elm" }
        , view = view
        , update = \_ model -> model
        }
