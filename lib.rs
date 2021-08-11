#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod CadavericKidneyWaitlist {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct CadavericKidneyWaitlist {
        /// Stores details of patient, patient_alert_status and patient_waitlist_rank. patienta_alert_status is used 
        /// to act as an indicator within the prototype as alert sent to the patient on availability of the cadaveric 
        /// organ for which the patient is waiting. When the organ has been allocated to anyone in the list, the
        /// patient_waitlist_rank for all patients will be updated and the patient_alert_status will be flipped.
        /// In the actual implementation after the prototype, a patient will be alerted by
        /// contacting his phone via sms and email.
        /// Only for this prototype string members are also declared as i32 but from the nameing
        /// convention one can infer they would be strings. This was done because of time
        /// constraints
        str_patient_id_type: i32,
        str_patient_id: i32,
        str_patient_name: i32,
        str_patient_gender: i32,
        patient_age: i32,
        str_patient_blood_group: i32,
        str_patient_phone_number: i32,
        str_patient_email_address: i32,
        str_patient_address: i32,
        patient_alert_status: bool,
        patient_waitlist_rank: i32,
    }


    impl CadavericKidneyWaitlist {

        /// Constructor that initializes the values to default values.
        ///
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                str_patient_id_type: 0,
                str_patient_id: 0,
                str_patient_name: 0,
                str_patient_gender: 0,
                patient_age: 0,
                str_patient_blood_group: 0,
                str_patient_phone_number: 0,
                str_patient_email_address: 0,
                str_patient_address: 0,
                patient_alert_status: false,
                patient_waitlist_rank: 0,
            }
        }

        ///A message to explicitly allocate values for patient details after the contract is
        ///instantiated
        #[ink(message)]
        pub fn patient_details_setter(&mut self, new_patient_id_type: i32, new_patient_id: i32, 
                                      new_patient_name: i32, new_patient_gender: i32, 
                                      new_patient_age: i32, new_patient_blood_group: i32, 
                                      new_patient_phone_number: i32, new_patient_email_address: i32,
                                      new_patient_address: i32, new_patient_alert_status: bool, 
                                      new_patient_waitlist_rank: i32) {
            self.str_patient_id_type = new_patient_id_type;
            self.str_patient_id = new_patient_id;
            self.str_patient_name = new_patient_name;
            self.str_patient_gender = new_patient_gender;
            self.patient_age = new_patient_age;
            self.str_patient_blood_group = new_patient_blood_group;
            self.str_patient_phone_number = new_patient_phone_number;
            self.str_patient_email_address = new_patient_email_address;
            self.str_patient_address = new_patient_address;
            self.patient_alert_status = new_patient_alert_status;
            self.patient_waitlist_rank = new_patient_waitlist_rank;
        }

        ///Returns the current value of patient_alert_status bool
        #[ink(message)]
        pub fn get_patient_alert_status(&self) -> bool {
            self.patient_alert_status
        }

        ///Sets the patient alert status. In prototype this value is set. In actual implementation
        ///patient will be contacted via email and sms
        #[ink(message)]
        pub fn set_patient_alert_status(&mut self) {
            self.patient_alert_status = true;
        }

        ///Resets the patient alerts status. In prototype, after the organ has been allocated to
        ///any patient in the list the alert status will be reset.
        #[ink(message)]
        pub fn reset_patient_alert_status(&mut self) {
            self.patient_alert_status = false;
        }
    }

    ///Commenting all the tests
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let CadavericKidneyWaitlist = CadavericKidneyWaitlist::default();
            assert_eq!(CadavericKidneyWaitlist.get_patient_alert_status(), false);
        }
    }
}
