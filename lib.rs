#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod CadavericKidneyWaitlist {
    
    /// The waiting list stores details of patient, patient_alert_status and patient_waitlist_rank.
    /// patient_alert_status is used to act as an indicator within the prototype as alert sent to the patient on
    /// availability of the cadaveric organ for which the patient is waiting. When the organ has been allocated 
    /// to anyone in the list, th patient_waitlist_rank for all patients will be updated and the patient_alert_status 
    /// will be flipped. In the actual implementation after the prototype, a patient will be alerted by contacting his 
    /// phone via sms and email.
    /// Only for this prototype string members are also declared as i32 but from the naming convention one can infer 
    /// they would be strings. This was done because of time constraints
    
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract
    use ink_storage::{
        traits::{
           PackedLayout,
           SpreadLayout,
        },
        Lazy,
    };
    #[derive(
        Debug,
        Copy,
        Clone,
        PartialEq,
        Eq,
        scale::Encode,
        scale::Decode,
        SpreadLayout,
        PackedLayout,
     )]
     #[cfg_attr(
         feature = "std",
         derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
     )]
    pub struct Spreaded_cadaver_kidney_waitlist_rec
            {
            ///One record of waiting list patient stored as a struct
            str_patient_id_type: i32,
            str_patient_id: i32,
            str_patient_name: i32,
            str_patient_gender: i32,
            int_patient_age: i32,
            str_patient_blood_group: i32,
            str_patient_phone_number: i32,
            str_patient_email_address: i32,
            str_patient_address: i32,
            bool_patient_alert_status: bool,
            int_patient_waitlist_rank: i32,
            }

     #[derive(
         Debug,
         Clone,
         PartialEq,
         Eq,
         scale::Encode,
         scale::Decode,
         SpreadLayout,
         PackedLayout,
     )]
     #[cfg_attr(
         feature = "std",
         derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
     )]
    pub struct Packed_cadaver_kidney_waitlist_rec
    
    {
        ///Waiting list patient records stored a sstruct is made to store as packing storage to be
        ///used for vector later
        packed_cadaver_kidney_waitlist_rec: ink_storage::Pack<Spreaded_cadaver_kidney_waitlist_rec>,
    }

    #[ink(storage)]
    pub struct CadavericKidneyWaitlist {
        /// The whole waiting list is a Vector of Packed struct.
        vec_cadaver_kidney_waitlist: ink_storage::collections::Vec<Packed_cadaver_kidney_waitlist_rec>,
    }


    impl CadavericKidneyWaitlist {

        /// Constructor that initializes the values to default values.
        ///
        #[ink(constructor)]
         pub fn default() -> Self {
            Self {
                 vec_cadaver_kidney_waitlist: Default::default(),
            }
        }

        ///A message to add a patient to the waiting list after entering patient details
        #[ink(message)]
        pub fn add_patient_to_waitlist(&mut self, new_patient_id_type: i32, new_patient_id: i32, 
                                      new_patient_name: i32, new_patient_gender: i32, 
                                      new_patient_age: i32, new_patient_blood_group: i32, 
                                      new_patient_phone_number: i32, new_patient_email_address: i32,
                                      new_patient_address: i32, new_patient_alert_status: bool, 
                                      new_patient_waitlist_rank: i32) {
            let temp_packed_cadaver_kidney_waitlist_rec: Packed_cadaver_kidney_waitlist_rec = *self.vec_cadaver_kidney_waitlist.last();                                         
            temp_packed_cadaver_kidney_waitlist_rec.str_patient_id_type = new_patient_id_type;
            temp_packed_cadaver_kidney_waitlist_rec.str_patient_id = new_patient_id;
            temp_packed_cadaver_kidney_waitlist_rec.str_patient_name = new_patient_name;
            temp_packed_cadaver_kidney_waitlist_rec.str_patient_gender = new_patient_gender;
            temp_packed_cadaver_kidney_waitlist_rec.int_patient_age = new_patient_age;
            temp_packed_cadaver_kidney_waitlist_rec.str_patient_blood_group = new_patient_blood_group;
            temp_packed_cadaver_kidney_waitlist_rec.str_patient_phone_number = new_patient_phone_number;
            temp_packed_cadaver_kidney_waitlist_rec.str_patient_email_address = new_patient_email_address; 
            temp_packed_cadaver_kidney_waitlist_rec.str_patient_address = new_patient_address;
            temp_packed_cadaver_kidney_waitlist_rec.bool_patient_alert_status = new_patient_alert_status;
            temp_packed_cadaver_kidney_waitlist_rec.int_patient_waitlist_rank = new_patient_waitlist_rank;
            self.vec_cadaver_kidney_waitlist.push(temp_packed_cadaver_kidney_waitlist_rec)
        }

        }
}
