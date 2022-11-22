use crate::domain::subscriber_name::SubscriberName;
use crate::domain::SubscriberEmail;
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
