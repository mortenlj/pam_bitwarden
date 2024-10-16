pam_bitwarden
=============

A PAM module to unlock your Bitwarden session with your password.

This assumes your bitwarden master password is the same as the password you use to log in to your computer.

Usage
-----

1. Download the latest release from the `releases page`_, and place it somewhere sensible, for instance ``/usr/local/lib/libpam_bitwarden.so``.
2. Add the following line to a suitable PAM configuration file.
   For instance, to unlock Bitwarden when you log in using sddm, add the following line to ``/etc/pam.d/sddm``::

    session	optional 	/usr/local/lib/pam_bitwarden/libpam_bitwarden.so

3. Log out and log back in. ``bw status`` should now show that your vault is unlocked.

.. _releases page: releases
