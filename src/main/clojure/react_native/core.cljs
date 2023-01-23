(ns react-native.core
  (:require ["react-native-web" :as rnw]
            [reagent.core :as reagent]))

(defn get-react-property [name]
  (aget rnw name))

(defn adapt-class [class]
  (when class
    (reagent/adapt-react-class class)))

(defn get-class [name]
  (adapt-class (get-react-property name)))

(def view (get-class "View"))
(def text (get-class "Text"))
(def image (get-class "Image"))
(def touchable-highlight (get-class "TouchableOpacity"))
(def scroll-view (get-class "ScrollView"))
(def text-input (get-class "TextInput"))

(def flat-list (get-class "FlatList"))
(def touchable-without-feedback (get-class "TouchableWithoutFeedback"))
(def touchable-opacity (get-class "TouchableOpacity"))
(defn use-window-dimensions [])
(defn dismiss-keyboard! [])
(def modal (get-class "Modal"))