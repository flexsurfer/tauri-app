(ns app.core
  (:require
   ["@tauri-apps/api/tauri" :as tauri]
   [goog.dom :as gdom]
   [reagent.core :as r]
   [reagent.dom :as dom]
   #_[quo2.core :as quo]
   #_[status-im2.contexts.quo-preview.buttons.button :as quo-preview.buttons]))

(defonce *name (r/atom ""))
(defonce *message (r/atom ""))

(def root
  (let [
        handle-input (fn [new-value]
                       (reset! *name new-value))
        greet! (fn [name]
                 ;;  Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                 (-> (.invoke tauri "greet" #js {:name name})
                     (.then (fn [res]
                              (reset! *message res)))))
        sha3! (fn [val]
                 ;;  Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                 (-> (.invoke tauri "sha3" #js {:value val})
                     (.then (fn [res]
                              (reset! *message res)))))]
    (fn []
      [:div.container
       [:h1 "Welcome to Tauri! OMG shadow-cljs hot reload from the box!!!"]

       [:div.row
        [:a {:href "https://tauri.app" :target "_blank"}
         [:img {:src "/tauri.svg" :class "logo tauri" :alt "Tauri logo"}]]
        [:a {:href "https://clojurescript.org" :target "_blank"}
         [:img {:src "/cljs.svg" :class "logo tauri" :alt "ClojureScript logo"}]]]

       [:p "Click on the Tauri, ClojureScript logos to learn more."]

       [:div.row
        [:input {:type "text"
                 :id "greet-input"
                 :on-change #(handle-input (.. % -target -value))
                 :placeholder "Enter a name..."}]
        [:button {:type "button" :on-click #(greet! @*name)} "Greet"]
        [:button {:type "button" :on-click #(sha3! @*name)} "Sha3 Name"]]
       #_[:div {:style {:display :flex :flex-direction :column}}
        [quo/text "Quo2 text"]
        [:div {:style {:display :flex :flex-direction :row :justify-content :center}}
         [quo/button {} "Quo2 button"]]]
       [:p "OMG34" @*message]])))

#_(defn root-preview []
  [:div {:style {:display :flex :flex-direction :column}}
   [quo-preview.buttons/cool-preview]])

(defn mount-root
  "Mount root component."
  {:dev/after-load true}
  []
  (some->>
    (gdom/getElement "root")
    (dom/render [root])))

(defn main
  "Application entry point."
  {:export true}
  [& _args]
  (mount-root))
