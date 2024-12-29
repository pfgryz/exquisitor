workspace "Name" "Description" {
    
    !identifiers hierarchical
    
    model {
        user = person "Użytkownik"
        advancedUser = person "Zaawansowany użytkownik"
        exquisitor = softwareSystem "Exquisitor" {
            app = container "exquisitor-app" {
                description "Aplikacja przeglądarkowa"
            }
            cli = container "exquisitor-cli" {
                description "Aplikacja konsolowa"
                tags "Cli"
            }
            core = container "exquisitor-core" {
                description "Główna biblioteka"
                tags "Library"
                
                needleman = component "ZmodyfikowanyalgorytmNeedlemana-Wunscha"
                kmer = component "Metoda zanurzeń k-merów"
                neural = component "Sztuczna sieć neuronowa"
            }
            appdb = container "exquisitor-db" {
                description "Baza danych zleceń aplikacji przeglądarkowej"
                tags "Database"
            }
        }
        blast = softwareSystem "BLASTn" {
            tags "Database"
        }

        # Container
        user -> exquisitor.app "Korzysta z"
        advancedUser -> exquisitor.cli "Korzysta z"
        exquisitor.app -> exquisitor.appdb "Odczytuje z i zapisuje do"
        exquisitor.app -> exquisitor.cli "Korzysta z"
        exquisitor.cli -> exquisitor.core "Korzysta z"
        exquisitor.core -> blast "Komunikuje się z"
    }
    
    views {
        systemContext exquisitor "Diagram1"{
            include *
            autolayout tb
        }
        
        container exquisitor "Diagram2" {
            include *
            autolayout tb
        }
        
        component exquisitor.core "Diagram3" {
            include *
            autolayout tb
        }
        
        properties {
            "structurizr.tooltips" "false"
            "structurizr.metadata" "false"
            "structurizr.title" "false"
        }
        
        styles {
            element "Person" {
                shape person
            }
            element "Database" {
                shape cylinder
            }
            element "Element" {
                fontsize 32
                background #1168bd
                color #ffffff
                shape RoundedBox
            }
            
            relationship "Relationship" {
                fontsize 32
                color #1168bd
            }
        }
        
        terminology {
            person "osoba"
            softwareSystem "system oprogramowania"
            container "kontener"
            component "komponent"
            deploymentNode "węzeł wdrożeniowy"
            infrastructureNode "węzeł infrastrukturalny"
            relationship "relacja"
        }
    }
    
}