#[cfg(test)]
mod test {
  use crate::{parse, ParseError};

  #[test]
  fn test() {
    let booking_ticket = r#"
ContextMap TicketBooking {
  Reservation -> Cinema;
  Reservation -> Movie;
  Reservation -> User;
}

Aggregate Reservation {
  Entity Ticket, Reservation;
}

Entity Reservation  {
  Struct {
    id: String;
    token: UUID;
    status: ReservationStatus = ReservationStatus.OPEN;
    expiresAt: LocalDateTime;
    createdAt: LocalDateTime;
    screeningId: String;
    screeningStartTime: LocalDateTime;
    name: String;
    surname: String;
    tickets: Set<Ticket>;
    totalPrice: BigDecimal;
  }
}

Entity Ticket  {}

Aggregate Cinema {
  Entity Cinema, ScreeningRoom, Seat;
}

Entity Cinema { }
Entity ScreeningRoom { }
Entity Seat { }

Aggregate Movie {
  Entity Movie, Actor, Publisher;
}

Entity Movie { }
Entity Actor { }
Entity Publisher { }

Aggregate User {
  Entity User;
}

Entity User {
  Struct {
    id: UUID;
    mobile: String;
    email: String;
    username: String;
    password: String;
    address: String;
  }
}

ValueObject Payment {
  id: UUID;
  payment_type: PaymentType;
  description: String;
}

ValueObject Price { }
ValueObject Notifications { }
"#;

    match parse(booking_ticket) {
      Ok(_) => {}
      Err(err) => {
        if let ParseError { kind, .. } = err {
          println!("Error: {:?}", kind);
        }
      }
    }

  }
}
