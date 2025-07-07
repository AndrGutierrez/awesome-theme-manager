use gtk::prelude::*;
use gtk::{ButtonsType, MessageDialog, MessageType, ResponseType};

/// A reusable confirmation dialog component.
///
/// This struct wraps a `gtk::MessageDialog` to provide a simple,
/// async interface for asking the user for confirmation.
pub struct ConfirmationDialog {
    inner: MessageDialog,
}

impl ConfirmationDialog {
    /// Creates a new confirmation dialog.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent window to which this dialog is transient.
    /// * `title` - The text to display in the dialog's title bar.
    /// * `message` - The primary message text to display in the dialog.
    ///
    /// # Returns
    ///
    /// A new instance of `ConfirmationDialog`.
    pub fn new(parent: &impl IsA<gtk::Window>, title: &str, message: &str) -> Self {
        let dialog = MessageDialog::builder()
            .transient_for(parent)
            .modal(true)
            .buttons(ButtonsType::OkCancel) // Use standard OK/Cancel buttons
            .message_type(MessageType::Question)
            .title(title)
            .text(message)
            .build();

        // The builder with `ButtonsType::OkCancel` automatically adds buttons
        // that return `ResponseType::Ok` and `ResponseType::Cancel`.

        Self { inner: dialog }
    }

    /// Runs the dialog and waits for the user's response.
    ///
    /// This method displays the dialog and returns a `Future` that resolves
    /// when the user clicks one of the buttons or closes the dialog.
    ///
    /// # Returns
    ///
    /// A `gtk::ResponseType` enum value corresponding to the button clicked.
    /// This will typically be `ResponseType::Ok` or `ResponseType::Cancel`.
    pub async fn run(&self) -> ResponseType {
        // `run_future` is the modern, asynchronous way to run a dialog.
        let response = self.inner.run_future().await;

        // The dialog is automatically hidden by GTK after a response is received,
        // but it's good practice to explicitly destroy it to free resources.
        self.inner.destroy();

        response
    }
}

