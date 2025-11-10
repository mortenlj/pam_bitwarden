# action will be run in python3 container
FROM python:3@sha256:97aa8cc0b87a4a312a294d2d4d7b20f6e2a21ed6d4e64ef08c03088c4aa9890f
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
