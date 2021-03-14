table! {
    nouns (id_noun) {
        id_noun -> Int4,
        gender -> Bpchar,
        animate -> Bool,
        sing_nominative -> Nullable<Varchar>,
        sing_genitive -> Nullable<Varchar>,
        sing_dative -> Nullable<Varchar>,
        sing_accusative -> Nullable<Varchar>,
        sing_instrumental -> Nullable<Varchar>,
        sing_prepositional -> Nullable<Varchar>,
        sing_locative -> Nullable<Varchar>,
        plur_nominative -> Nullable<Varchar>,
        plur_genitive -> Nullable<Varchar>,
        plur_dative -> Nullable<Varchar>,
        plur_accusative -> Nullable<Varchar>,
        plur_instrumental -> Nullable<Varchar>,
        plur_prepositional -> Nullable<Varchar>,
        plur_locative -> Nullable<Varchar>,
    }
}
