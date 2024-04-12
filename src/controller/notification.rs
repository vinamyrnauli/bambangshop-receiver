use rocket::serde::json::Json;

use bambangshop_receiver::Result;
use crate::model::notificiation::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::service::notification::NotificationService;