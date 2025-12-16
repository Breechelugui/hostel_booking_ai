use hostel_booking_ai::{Booking, BookingAI};

fn main() {
    let ai = BookingAI::new();
    let room = ai.suggest_room("quiet room");
    let booking = Booking::new(1, "John Doe".to_string(), room, "2024-01-01".to_string(), "2024-01-03".to_string());
    
    println!("Booking created: {} in room {}", booking.guest_name, booking.room_number);
}
