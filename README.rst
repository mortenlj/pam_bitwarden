pam_bitwarden
=============

A PAM module to unlock your Bitwarden session with your password.

This assumes your bitwarden master password is the same as the password you use to log in to your computer.

Risks vs. Rewards
-----------------

Before using this module, make sure you understand the risks and rewards, and are comfortable with them.

This module works by adding a ``BW_SESSION`` environment variable to your session when logging in.
This environment variable contains the session key needed to access your Bitwarden vault.
Any shell or process that inherits this environment variable will be able to access your Bitwarden vault without needing to enter your master password, unless you have explicitly locked it after logging in.

Usage
-----

1. Download the latest release from the `releases page`_, and place it somewhere sensible, for instance ``/usr/local/lib/libpam_bitwarden.so``.
2. Add the following line to a suitable PAM configuration file.
   For instance, to unlock Bitwarden when you log in using sddm, add the following line to ``/etc/pam.d/sddm``::

    session	optional 	/usr/local/lib/pam_bitwarden/libpam_bitwarden.so

3. Log out and log back in. ``bw status`` should now show that your vault is unlocked.

.. _releases page: releases
