# action will be run in python3 container
FROM python:3@sha256:bdc6c1e5773e8f4f2e8ec47b2fb666daec8bed64f78edd96d4f2c6a91865b14f
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
