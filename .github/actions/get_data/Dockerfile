# action will be run in python3 container
FROM python:3@sha256:d29cf0828933ed271be9234ca2c2d578c16f2911451418aacc4525ac04ac7114
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
